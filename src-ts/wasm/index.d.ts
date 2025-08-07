/* tslint:disable */
/* eslint-disable */
export function decode(bytes: Uint8Array, bpm?: number | null): NoteEvent[] | undefined;
export class NoteEvent {
  private constructor();
  free(): void;
  code: number;
  start: number;
  end: number;
}

