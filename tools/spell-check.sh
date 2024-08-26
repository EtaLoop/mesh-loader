#!/bin/bash
# shellcheck disable=SC2046
set -euo pipefail
IFS=$'\n\t'
cd "$(dirname "$0")"/..

# Usage:
#    ./tools/spell-check.sh

check_diff() {
    if [[ -n "${CI:-}" ]]; then
        if ! git --no-pager diff --exit-code "$@"; then
            should_fail=1
        fi
    else
        if ! git --no-pager diff --exit-code "$@" &>/dev/null; then
            should_fail=1
        fi
    fi
}
error() {
    if [[ -n "${GITHUB_ACTIONS:-}" ]]; then
        echo "::error::$*"
    else
        echo >&2 "error: $*"
    fi
    should_fail=1
}
warn() {
    if [[ -n "${GITHUB_ACTIONS:-}" ]]; then
        echo "::warning::$*"
    else
        echo >&2 "warning: $*"
    fi
}

project_dictionary=.github/.cspell/project-dictionary.txt
has_rust=''
if [[ -n "$(git ls-files '*Cargo.toml')" ]]; then
    has_rust='1'
    dependencies=''
    for manifest_path in $(git ls-files '*Cargo.toml'); do
        if [[ "${manifest_path}" != "Cargo.toml" ]] && ! grep -Eq '\[workspace\]' "${manifest_path}"; then
            continue
        fi
        metadata=$(cargo metadata --format-version=1 --all-features --no-deps --manifest-path "${manifest_path}")
        for id in $(jq <<<"${metadata}" '.workspace_members[]'); do
            dependencies+="$(jq <<<"${metadata}" ".packages[] | select(.id == ${id})" | jq -r '.dependencies[].name')"$'\n'
        done
    done
    # shellcheck disable=SC2001
    dependencies=$(sed <<<"${dependencies}" 's/[0-9_-]/\n/g' | LC_ALL=C sort -f -u)
fi
config_old=$(<.cspell.json)
config_new=$(grep <<<"${config_old}" -v '^ *//' | jq 'del(.dictionaries[])' | jq 'del(.dictionaryDefinitions[])')
trap -- 'echo "${config_old}" >.cspell.json; echo >&2 "$0: trapped SIGINT"; exit 1' SIGINT
echo "${config_new}" >.cspell.json
if [[ -n "${has_rust}" ]]; then
    dependencies_words=$(npx <<<"${dependencies}" -y cspell stdin --no-progress --no-summary --words-only --unique || true)
fi
all_words=$(npx -y cspell --no-progress --no-summary --words-only --unique $(git ls-files | (grep -v "${project_dictionary//\./\\.}" || true)) || true)
echo "${config_old}" >.cspell.json
trap - SIGINT
cat >.github/.cspell/rust-dependencies.txt <<EOF
// This file is @generated by $(basename "$0").
// It is not intended for manual editing.
EOF
if [[ -n "${dependencies_words:-}" ]]; then
    echo $'\n'"${dependencies_words}" >>.github/.cspell/rust-dependencies.txt
fi
check_diff .github/.cspell/rust-dependencies.txt

echo "+ npx -y cspell --no-progress --no-summary \$(git ls-files)"
if ! npx -y cspell --no-progress --no-summary $(git ls-files); then
    error "spellcheck failed: please fix uses of below words or add to ${project_dictionary} if correct"
    echo >&2 "======================================="
    (npx -y cspell --no-progress --no-summary --words-only $(git ls-files) || true) | LC_ALL=C sort -f -u >&2
    echo >&2 "======================================="
    echo >&2
fi

# Make sure the project-specific dictionary does not contain duplicated words.
for dictionary in .github/.cspell/*.txt; do
    if [[ "${dictionary}" == "${project_dictionary}" ]]; then
        continue
    fi
    dup=$(sed '/^$/d' "${project_dictionary}" "${dictionary}" | LC_ALL=C sort -f | uniq -d -i | (grep -v '//.*' || true))
    if [[ -n "${dup}" ]]; then
        error "duplicated words in dictionaries; please remove the following words from ${project_dictionary}"
        echo >&2 "======================================="
        echo >&2 "${dup}"
        echo >&2 "======================================="
        echo >&2
    fi
done

# Make sure the project-specific dictionary does not contain unused words.
if [[ -n "${REMOVE_UNUSED_WORDS:-}" ]]; then
    grep_args=()
    for word in $(grep -v '//.*' "${project_dictionary}" || true); do
        if ! grep <<<"${all_words}" -Eq -i "^${word}$"; then
            # TODO: single pattern with ERE: ^(word1|word2..)$
            grep_args+=(-e "^${word}$")
        fi
    done
    if [[ ${#grep_args[@]} -gt 0 ]]; then
        warn "removing unused words from ${project_dictionary}"
        res=$(grep -v "${grep_args[@]}" "${project_dictionary}")
        echo "${res}" >"${project_dictionary}"
    fi
else
    unused=''
    for word in $(grep -v '//.*' "${project_dictionary}" || true); do
        if ! grep <<<"${all_words}" -Eq -i "^${word}$"; then
            unused+="${word}"$'\n'
        fi
    done
    if [[ -n "${unused}" ]]; then
        warn "unused words in dictionaries; please remove the following words from ${project_dictionary}"
        echo >&2 "======================================="
        echo >&2 -n "${unused}"
        echo >&2 "======================================="
        echo >&2
    fi
fi

if [[ -n "${should_fail:-}" ]]; then
    exit 1
fi
