### Everything is in the Makefile:

```sh
# Classic git add + commit + push ALL at once (prompts for comment)
make push

# Build locally
make build

# Put release files into more convenient `/release` folder
make release_local

# Upload to distant linux machine using SSH 
make release_ssh_<SSH_ADDRESS_or_SSH_NAME>

# Routine to increment new version + git tag + git status + git push
make push_new_release
```

### To publish to crates.io

```sh
cargo publish --dry-run
```

