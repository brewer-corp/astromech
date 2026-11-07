/**
 * Engineering materials: trader-style inventory from the journal.
 */
import type { Inventory } from "./api/types/Inventory";
import { call } from "./transport";

export type { Cell } from "./api/types/Cell";
export type { FarmLine } from "./api/types/FarmLine";
export type { GroupView } from "./api/types/GroupView";
export type { HgeHere as Here } from "./api/types/HgeHere";
export type { Inventory } from "./api/types/Inventory";
export type { KindView } from "./api/types/KindView";
export type { SourceNote } from "./api/types/SourceNote";

export async function loadMaterials(): Promise<Inventory> {
  return await call("materials_inventory");
}
