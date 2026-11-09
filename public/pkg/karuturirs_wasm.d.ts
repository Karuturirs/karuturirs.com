/* tslint:disable */
/* eslint-disable */

export function kids_answer_choice(_choice_idx: number, val: string): void;

export function kids_finish_game(): void;

export function kids_keep_going(): void;

export function kids_next_question(): void;

export function kids_quit_game(): void;

export function kids_replay_game(): void;

export function kids_set_diff(diff: string): void;

export function kids_set_round_len(len: number): void;

export function kids_set_style(style: string): void;

export function kids_show_home(): void;

export function kids_speak_question(): void;

export function kids_start_game(game_id: string): void;

export function kids_tap_spell_tile(tile_idx: number, ch_str: string): void;

export function kids_tick_clock(): void;

export function kids_toggle_sound(): void;

export function kids_toggle_tech(): void;

export function kids_undo_spell(): void;

export function navigate_to(path: string): void;

export function start(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly kids_answer_choice: (a: number, b: number, c: number) => void;
    readonly kids_finish_game: () => void;
    readonly kids_keep_going: () => void;
    readonly kids_next_question: () => void;
    readonly kids_quit_game: () => void;
    readonly kids_replay_game: () => void;
    readonly kids_set_diff: (a: number, b: number) => void;
    readonly kids_set_round_len: (a: number) => void;
    readonly kids_set_style: (a: number, b: number) => void;
    readonly kids_show_home: () => void;
    readonly kids_speak_question: () => void;
    readonly kids_start_game: (a: number, b: number) => void;
    readonly kids_tap_spell_tile: (a: number, b: number, c: number) => void;
    readonly kids_tick_clock: () => void;
    readonly kids_toggle_sound: () => void;
    readonly kids_toggle_tech: () => void;
    readonly kids_undo_spell: () => void;
    readonly navigate_to: (a: number, b: number) => void;
    readonly start: () => void;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
