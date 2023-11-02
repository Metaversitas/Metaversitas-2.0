import allPrgrams from '@/_content/all-programs.json'

export type TProgram = {
  readonly title: string
  readonly description: string
}

export type TPrograms = {
  readonly programs: TProgram[]
}

export default allPrgrams as unknown as TPrograms
