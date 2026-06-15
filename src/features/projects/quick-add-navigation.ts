import { goto } from "$app/navigation";
import { toast } from "svelte-sonner";
import type { QuickAddResult } from "@/api/shell";
import { diagramHref } from "@/features/diagrams/diagram-types";

type PrepareNavigation = (result: QuickAddResult) => Promise<boolean>;

let prepareNavigation: PrepareNavigation = async () => true;

export function registerQuickAddNavigation(
  prepare: PrepareNavigation,
) {
  prepareNavigation = prepare;
  return () => {
    if (prepareNavigation === prepare) {
      prepareNavigation = async () => true;
    }
  };
}

export async function openQuickAddResult(result: QuickAddResult) {
  if (!(await prepareNavigation(result))) {
    toast.error("The new diagram was created, but the current diagram could not be saved.");
    return;
  }

  await goto(diagramHref(result.project, result.path));

  if (result.warning) {
    toast.warning("Diagram created without AI content", {
      description: result.warning,
    });
  }
}
