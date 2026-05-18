/* tslint:disable */
/* eslint-disable */

export class Komment {
    free(): void;
    [Symbol.dispose](): void;
    create_discussion(repo_owner: string, repo_name: string, category_name: string, title: string, body: string): Promise<any>;
    delete_comment(comment_id: string): Promise<any>;
    fetch_discussion(): Promise<any>;
    constructor(config: any);
    post_comment(discussion_id: string, body: string): Promise<any>;
    render(element_id: string, data: any): void;
    update_comment(comment_id: string, body: string): Promise<any>;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_komment_free: (a: number, b: number) => void;
    readonly komment_create_discussion: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number) => any;
    readonly komment_delete_comment: (a: number, b: number, c: number) => any;
    readonly komment_fetch_discussion: (a: number) => any;
    readonly komment_new: (a: any) => [number, number, number];
    readonly komment_post_comment: (a: number, b: number, c: number, d: number, e: number) => any;
    readonly komment_render: (a: number, b: number, c: number, d: any) => [number, number];
    readonly komment_update_comment: (a: number, b: number, c: number, d: number, e: number) => any;
    readonly wasm_bindgen__convert__closures_____invoke__hf39fca293b0a8846: (a: number, b: number, c: any) => [number, number];
    readonly wasm_bindgen__convert__closures_____invoke__h28b7bbeccbc6f8b7: (a: number, b: number, c: any, d: any) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_destroy_closure: (a: number, b: number) => void;
    readonly __externref_table_dealloc: (a: number) => void;
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
