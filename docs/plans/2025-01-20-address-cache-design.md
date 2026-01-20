# Address Cache & Download Feature Design

**Date:** 2025-01-20
**Status:** Approved

## Overview

Add functionality to download real addresses from OpenAddresses.io, cache them locally by state, and use them for generating test data with variance patterns.

## CLI Interface

### New `download` subcommand

```bash
rust-faker download IL CA TX          # Download specific states
rust-faker download --all             # Download all 50 states
rust-faker download --list            # Show cache status
rust-faker download IL --limit 50000  # Custom address count per state
rust-faker download IL --force        # Re-download even if cached
```

### Updated `addresses` subcommand

```bash
# Existing modes (unchanged)
rust-faker addresses -c 1000 -o out.csv              # Generate fake
rust-faker addresses --input file.csv -o out.csv    # Load from file

# New cached mode
rust-faker addresses --state IL -o out.csv           # All cached IL addresses
rust-faker addresses --state IL -c 1000 -o out.csv   # Sample 1000 from IL
rust-faker addresses --state IL,CA,TX -c 5000 -o out.csv  # Sample across states
rust-faker addresses --state all -c 10000 -o out.csv # Sample from all cached
```

**Mutual exclusivity:** `--input`, `--state`, and standalone `--count` (fake generation) are mutually exclusive.

## Cache Structure

**Location:** `~/.rust-faker/cache/addresses/`

```
~/.rust-faker/
└── cache/
    └── addresses/
        ├── manifest.json
        ├── IL.csv
        ├── CA.csv
        └── TX.csv
```

### Manifest format

```json
{
  "version": 1,
  "states": {
    "IL": {
      "downloaded_at": "2025-01-20T10:30:00Z",
      "source_url": "https://data.openaddresses.io/...",
      "record_count": 10000,
      "file_hash": "sha256:abc123..."
    }
  }
}
```

### Cache file format

Standard CSV matching existing format:

```csv
address1,address2,city,state,zip
123 Main St,,Springfield,IL,62701
456 Oak Ave,Apt 2,Chicago,IL,60601
```

## Data Source

### OpenAddresses.io

Regional zip files available at:
- `https://data.openaddresses.io/openaddr-collected-us_northeast.zip`
- `https://data.openaddresses.io/openaddr-collected-us_midwest.zip`
- `https://data.openaddresses.io/openaddr-collected-us_south.zip`
- `https://data.openaddresses.io/openaddr-collected-us_west.zip`

Each regional zip contains state folders with CSV files.

### Download process

1. Check cache - skip if state cached and not forcing refresh
2. Download regional zip - fetch appropriate regional archive
3. Extract state CSV - pull relevant state's CSV from archive
4. Sample & normalize - take `--limit` random rows (default: 10,000), normalize columns
5. Write cache - save as `~/.rust-faker/cache/addresses/{STATE}.csv`
6. Update manifest - record download timestamp, source, row count

### State-to-region mapping

Hardcoded lookup table mapping each state abbreviation to its OpenAddresses region.

## Runtime Behavior

### Loading from cache

1. Parse state list - split `IL,CA,TX` or handle `all` keyword
2. Validate cache - check each state exists; fail fast if any missing
3. Load addresses - read CSV files into memory
4. Sample if needed - if `--count` specified, randomly sample from combined pool
5. Apply variance - apply `--error-rate` patterns
6. Write output - write to output file

### Sampling across states

When multiple states specified with count limit:
- Load all addresses from requested states
- Shuffle combined pool
- Take first N addresses (random distribution across states)

### Auto-download behavior

When `--state` requests uncached state:
- Automatically attempt download
- If offline or download fails, show clear error with manual download instructions

### Error messages

```
Error: State 'IL' not cached. Run 'rust-faker download IL' first.
Error: Invalid state code 'XX'. Use two-letter state abbreviations.
Error: Cannot use --state and --input together. Choose one.
```

## Implementation

### New dependencies

```toml
[dependencies]
reqwest = { version = "0.11", features = ["blocking"] }
zip = "0.6"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
dirs = "5.0"
sha2 = "0.10"
```

### New files

- `src/cache.rs` - Cache management (paths, manifest read/write)
- `src/download.rs` - Download logic (fetch, extract, sample)
- `src/regions.rs` - State-to-region mapping table

### Modified files

- `src/main.rs` - Add `Download` subcommand, add `--state` flag to `Addresses`
- `src/generators/addresses.rs` - Add `load_addresses_from_cache()` function

## Testing

- Unit tests for state validation, manifest parsing, region mapping
- Integration tests using mock HTTP responses or test fixtures
- Manual testing with real OpenAddresses downloads

## Storage Estimates

- ~1-2 MB per state at 10,000 addresses
- ~75-100 MB for all 50 states at default limit
