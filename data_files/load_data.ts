import { Glob } from "bun";

export interface Player {
    id: number,
    timestamp: number,
    formation: string,
    untradeable: boolean,
    assetId: number,
    rating: number,
    itemType: string,
    resourceId: number,
    owners: number,
    discardValue: number,
    itemState: string,
    cardsubtypeid: number,
    lastSalePrice: number,
    injuryType: string,
    injuryGames: number,
    preferredPosition: string,
    contract: number,
    teamid: number,
    rareflag: number,
    playStyle: number,
    leagueId: number,
    assists: number,
    lifetimeAssists: number,
    loans: number,
    loansInfo: {
        loanType: string,
        loanValue: number
    },
    loyaltyBonus: number,
    pile: number,
    nation: number,
    marketDataMinPrice: number,
    marketDataMaxPrice: number,
    resourceGameYear: number,
    guidAssetId: string,
    groups: number[],
    attributeArray: number[],
    statsArray: number[],
    lifetimeStatsArray: number[],
    skillmoves: number,
    weakfootabilitytypecode: number,
    attackingworkrate: number,
    defensiveworkrate: number,
    preferredfoot: number,
    possiblePositions: string[],
    gender: number,
    baseTraits: number[],
    iconTraits: number[],

    __name:string
}

export interface MD {
    c?: string,
    f: string,
    id: number,
    l: string,
    r: number
}

export interface PlayerData {
    club: Player[],
    meta_data: Map<number, MD>,
    name_to_player: Map<string, Player>,
}

let PLAYER_DATA: PlayerData | null = null;

function read_file(fn: string, output_arr: Player[]): Promise<any> {
    return Bun.file(fn).json().then((json) => {
        for (let i of json.itemData) {
            output_arr.push(i);
        }
    });
}

function read_players_md_file(fn: string, out_map: Map<number, MD>): Promise<any> {
    return Bun.file(fn).json().then((json) => {
        for (let d of json.LegendsPlayers) {
            out_map.set(d.id, d);
        }
        for (let d of json.Players) {
            out_map.set(d.id, d);
        }
    });
}

export async function load_data(): Promise<PlayerData> {
    if (PLAYER_DATA == null) {
        PLAYER_DATA = {
            club: [],
            meta_data: new Map(),
            name_to_player: new Map()
        };
        const ps: Promise<any>[] = [];
        const glob = new Glob("club*.json");
        const folder = "./data";
        for (const file of glob.scanSync(folder)) {
            ps.push(read_file(`${folder}/${file}`, PLAYER_DATA.club));
        }
        ps.push(read_players_md_file("data/players.json", PLAYER_DATA.meta_data));

        await Promise.all(ps);
        console.log("loaded", PLAYER_DATA.club.length, "players");

        // creating by_name & to_name maps
        for (const player of PLAYER_DATA.club) {
            const n: MD | undefined = PLAYER_DATA.meta_data.get(player.assetId);
            if (n) {
                const name = n.c || `${n.f} ${n.l}`;
                PLAYER_DATA.name_to_player.set(name.toLowerCase(), player);
                player.__name = name;
            }
        }

        // sorting players by rating
        PLAYER_DATA.club.sort((a, b) => {
            return a.rating - b.rating;
        });
    }

    return PLAYER_DATA;
}

export function get_player_by_name(name: string): Player | undefined {
    if (PLAYER_DATA == null) {
        return undefined;
    }
    return PLAYER_DATA.name_to_player.get(name.toLowerCase());
}