# fastasmash

A FASTA file CLI utility written in Rust.

## Installation

```bash
cargo build --release
# binary will be at ./target/release/fastasmash
```

## Usage

```
fastasmash --fasta-file <FILE> [--id <ID>] <COMMAND>
```

### Arguments

| Argument | Short | Description |
|---|---|---|
| `--fasta-file <FILE>` | `-f` | Path to the FASTA file (required) |
| `--id <ID>` | | Filter records whose ID contains `<ID>` (substring match) |

### Commands

#### `count`

Prints the number of records in the file.

```
fastasmash --fasta-file sequences.fasta count
```

Example output:

```
File sequences.fasta has 42 records
```

#### `header`

Prints the header line of every record (`>id description`).

```
fastasmash --fasta-file sequences.fasta header
```

Example output:

```
>seq1 Homo sapiens chromosome 1
>seq2 Homo sapiens chromosome 2
```

#### `view`

Prints complete records — header followed by sequence.

```
fastasmash --fasta-file sequences.fasta view
```

Example output:

```
>seq1 Homo sapiens chromosome 1
ACGTACGTACGT
>seq2 Homo sapiens chromosome 2
TGCATGCATGCA
```

## Filtering with `--id`

The `--id` flag filters records to those whose ID contains the given string (substring match). It can be combined with any command.

```bash
# count only records whose ID contains "chr1"
fastasmash --fasta-file sequences.fasta --id chr1 count

# view only matching records
fastasmash --fasta-file sequences.fasta --id seq1 view
```

Note: `--id seq1` will match `seq1`, `seq10`, `seq100`, etc.
