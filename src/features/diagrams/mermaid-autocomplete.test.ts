import { describe, expect, it } from "vitest";
import { getMermaidCompletions } from "./mermaid-autocomplete";

function completionsFor(source: string) {
  return getMermaidCompletions(source, source.length);
}

function labelsFor(source: string) {
  return completionsFor(source).map((item) => item.label);
}

describe("getMermaidCompletions", () => {
  it("suggests flowchart node and subgraph identifiers matching the prefix", () => {
    const labels = labelsFor(`flowchart TB
subgraph AI["Document processing pipeline"]
direction TB
AI_A["Read file"]
A`);

    expect(labels).toContain("AI");
    expect(labels).toContain("AI_A");
  });

  it("does not suggest flowchart labels", () => {
    const labels = labelsFor(`flowchart TB
subgraph AI["Document processing pipeline"]
AI_A["Read file"]
R`);

    expect(labels).not.toContain("Read");
    expect(labels).not.toContain("file");
    expect(labels).not.toContain("Document");
    expect(labels).not.toContain("processing");
    expect(labels).not.toContain("pipeline");
  });

  it("suggests flowchart identifiers found in relationships", () => {
    const labels = labelsFor(`flowchart LR
A-->Backend
B`);

    expect(labels).toEqual(["Backend"]);
  });

  it("ignores Mermaid comments", () => {
    const labels = labelsFor(`flowchart TB
%% GhostNode["Ignored"]
G`);

    expect(labels).not.toContain("GhostNode");
    expect(labels).toEqual(["graph"]);
  });

  it("extracts sequence participants and aliases but not message text", () => {
    const labels = labelsFor(`sequenceDiagram
participant User as Browser
actor API
Browser->>API: Request document
B`);

    expect(labels).toContain("Browser");
    expect(labels).not.toContain("Request");
    expect(labels).not.toContain("document");
  });

  it("extracts state identifiers from aliases and transitions", () => {
    const labels = labelsFor(`stateDiagram-v2
state "Document Processing" as Processing
[*] --> Idle
Idle --> Processing
P`);

    expect(labels).toContain("Processing");
  });

  it("extracts ERD entity identifiers from relationships and entity blocks", () => {
    const labels = labelsFor(`erDiagram
CUSTOMER ||--o{ ORDER : places
ORDER {
  string orderNumber
}
O`);

    expect(labels).toEqual(["ORDER"]);
  });

  it("does not suggest ERD fields or relationship labels", () => {
    const labels = labelsFor(`erDiagram
CUSTOMER ||--o{ ORDER : "places order"
ORDER {
  string orderNumber
}
o`);

    expect(labels).not.toContain("orderNumber");
    expect(labels).not.toContain("order");
  });

  it("does not suggest an exact identifier match", () => {
    const labels = labelsFor(`flowchart TB
B["Beta"]
A["Alpha"]
A-->B
A`);

    expect(labels).toEqual([]);
  });

  it("does not suggest an exact keyword match", () => {
    const labels = labelsFor(`sequenceDiagram
participant`);

    expect(labels).toEqual([]);
  });

  it("does not suggest an exact ERD keyword match", () => {
    const labels = labelsFor("erDiagram");

    expect(labels).toEqual([]);
  });

  it("does not suggest when the cursor is inside a token", () => {
    const source = `sequenceDiagram
participant User`;
    const cursorOffset = source.indexOf("participant") + "part".length;

    expect(getMermaidCompletions(source, cursorOffset)).toEqual([]);
  });
});
