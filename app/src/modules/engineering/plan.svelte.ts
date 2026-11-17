// A ship's build plan, shared by the two views of a ship: the Loadout's side panel edits it and its
// rows show the planned grades; the Plan view shows what it costs. It lives with the page, so a
// switch between the views keeps it, and each edit is evaluated once.
import {
  DEFAULT_ROLLS, dropShoppingList, evaluatePlan, savePlan, savedPlan, shareShoppingList,
  type Fitted, type Fleet, type Plan, type PlanResult, type Rolls, type Target,
} from "../../lib/engineering";

/** The top grade of a blueprint for a module (5 when the data has none). */
export function maxGrade(fleet: Fleet, m: Fitted, blueprint: string | undefined): number {
  return fleet.groups[m.group ?? ""]?.blueprints.find((b) => b.id === blueprint)?.max_grade ?? 5;
}

/**
 * The plan with the module's target changed by `patch`, or removed (`null`, or a blueprint the
 * module's group lacks). A new blueprint starts at its top grade; the experimental is kept from
 * the old target, else from the module.
 */
export function withTarget(plan: Plan, fleet: Fleet, m: Fitted, patch: Partial<Target> | null): Plan {
  const others = plan.targets.filter((t) => t.slot !== m.slot);
  const old = plan.targets.find((t) => t.slot === m.slot);
  let next: Target | null = null;
  if (patch) {
    const blueprint = patch.blueprint ?? old?.blueprint ?? m.engineering?.blueprint ?? "";
    const choice = fleet.groups[m.group ?? ""]?.blueprints.find((b) => b.id === blueprint);
    if (choice) {
      const top = choice.max_grade ?? 5;
      const grade = patch.grade ?? (old?.blueprint === blueprint ? old.grade : top);
      const experimental = patch.experimental !== undefined ? patch.experimental : (old?.experimental ?? m.engineering?.experimental ?? null);
      next = { slot: m.slot, blueprint, grade: Math.min(grade, top), experimental: experimental || null };
    }
  }
  return { ...plan, targets: next ? [...others, next] : others };
}

/**
 * What ADD TO PLAN does for a module: one grade up on the blueprint it has (the planned one when
 * the plan has a target, else the fitted one); `pick` when it has neither, so the commander
 * chooses a blueprint first; `top` when that blueprint is already at its top grade.
 */
export function gradeUp(fleet: Fleet, m: Fitted, t: Target | undefined): { add: Partial<Target> } | "pick" | "top" {
  const blueprint = t?.blueprint ?? m.engineering?.blueprint;
  if (!blueprint) return "pick";
  const from = t ? t.grade : (m.engineering?.grade ?? 0);
  if (from >= maxGrade(fleet, m, blueprint)) return "top";
  return { add: { blueprint, grade: from + 1 } };
}

/** The selected ship's plan and what it costs. Make it while a component starts (it has effects). */
export class ShipPlan {
  plan = $state<Plan | null>(null);
  result = $state<PlanResult | null>(null);
  /** The plan could not be worked out: why. */
  failed = $state<string | null>(null);
  #seq = 0;

  constructor(fleet: () => Fleet | null, shipId: () => number | undefined) {
    // A new ship (or the first load): restore its plan.
    $effect(() => {
      const id = shipId();
      if (id === undefined) {
        this.plan = null;
        this.result = null;
        return;
      }
      if (this.plan?.ship_id === id) return;
      this.plan = savedPlan(id);
      this.result = null;
      this.failed = null;
    });

    // Work it out on every edit and every fleet re-read (the inventory changes what is short).
    $effect(() => {
      const p = this.plan;
      void fleet();
      if (!p) return;
      const snapshot = $state.snapshot(p) as Plan;
      const mine = ++this.#seq;
      evaluatePlan(snapshot)
        .then((r) => {
          if (mine !== this.#seq) return; // a newer request (another ship, a later edit) is in flight
          this.result = r;
          shareShoppingList(r);
          this.failed = null;
        })
        .catch((e: unknown) => {
          if (mine !== this.#seq) return;
          this.result = null;
          this.failed = String(e);
          // A ship that left the fleet keeps no list in the core.
          if (this.failed.includes("not in the fleet")) dropShoppingList(snapshot.ship_id);
        });
    });
  }

  target(slot: string): Target | undefined {
    return this.plan?.targets.find((t) => t.slot === slot);
  }

  /** What the plan costs for one module, if it has a target. */
  cost(slot: string) {
    return this.result?.targets.find((t) => t.slot === slot);
  }

  set(fleet: Fleet, m: Fitted, patch: Partial<Target> | null) {
    if (!this.plan) return;
    this.#save(withTarget(this.plan, fleet, m, patch));
  }

  setRolls(i: number, v: number) {
    if (!this.plan || !Number.isFinite(v)) return;
    const rolls: Rolls = [...this.plan.rolls];
    rolls[i] = Math.max(1, Math.min(20, Math.round(v)));
    this.#save({ ...this.plan, rolls });
  }

  resetRolls() {
    if (this.plan) this.#save({ ...this.plan, rolls: DEFAULT_ROLLS });
  }

  #save(p: Plan) {
    this.plan = p;
    savePlan(p);
  }
}
