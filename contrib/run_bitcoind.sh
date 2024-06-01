#!/usr/bin/env bash
#
# Run local regtest `bitcoind` nodes.

set -euo pipefail

# RPC authentication username.
RPC_USER="user"
# RPC authentication password.
RPC_PASSWORD="password"

usage() {
    cat <<EOF
Usage:

    ./run_bitcoind.sh [COMMAND]

COMMAND
   - all                      Start all known bitcoind versions.
   - start [KNOWN_VERSION]    Start bitcoind nodes, defaults to v22.
   - stop                     Kill all bitcoind nodes using 'pkill bitcoind'.

KNOWN_VERSION
   - v17                Bitcoin Core v0.17.1
   - v22                Bitcoin Core v22.1

EOF
}

main() {
    local cmd="${1:-usage}"
    local version="${2:-v22}"

    # FIXME: This is a hackish way to get the help flag.
    if [ "$cmd" = "usage" ] || [ "$cmd" = "-h" ] || [ "$cmd" = "--help" ] || [ "$cmd" = "help" ]; then
        usage
        exit 0
    fi

    case $cmd in
        all)
            start "v17"
            start "v22"
            ;;

        start)
            start "$version"
            ;;

        stop)
            pkill bitcoind
            rm -rf "/tmp/rust-bitcoind-json-rpc-0.17.1/2/regtest/wallets" > /dev/null
            rm -rf "/tmp/rust-bitcoind-json-rpc-22.1/2/regtest/wallets" > /dev/null
            ;;
        *)
            usage
            say "Error: unknown command $cmd"
            ;;

    esac
}

start() {
    local version="$1"

    case $version in
        v17)
            local version_number="0.17.1"
            local version_id="171"
            run_bitcoind "$version" "$version_number" "$version_id"
            ;;

        v22)
            local version_number="22.1"
            local version_id="221"
            run_bitcoind "$version" "$version_number" "$version_id"
            ;;
        *)
            usage
            err "Error: unknown version $version"
            ;;
    esac
}

run_bitcoind() {
    local version="$1"          # eg, v22
    local version_number="$2"   # eg, 22.1
    local version_id="$3"       # eg, 221

    local test_dir="/tmp/rust-bitcoind-json-rpc-${version_number}"
    local bitcoind="/opt/bitcoin-${version_number}/bin/bitcoind"
    # RPC port number of the node we hit when testing (xyz49 where xyz is the bitcoind version identifier).
    local rpc_port="${version_id}49"

    if "$bitcoind" -version | grep -q "${version_number}"; then
        echo "Starting two bitcoind v${version_number} instances"
    else
        echo "Wrong bitcoind version, expected ${version_number}"
        "$bitcoind" -version
        exit 1
    fi

    rm -rf "${test_dir}"
    mkdir -p "${test_dir}/1" "${test_dir}/2"


    local block_filter_arg=""
    if echo "${version_number}" | grep -q "0\.\(19\|2\)"; then
        block_filter_arg="-blockfilterindex=1"
    fi

    local fallback_fee_arg=""
    if echo "${version_number}" | grep -q "2.\."; then
        fallback_fee_arg="-fallbackfee=0.00001000"
    fi

    "$bitcoind" -regtest $fallback_fee_arg $block_filter_arg \
                -datadir="${test_dir}/1" \
                -port="${version_id}48" \
                -server=0 \
                -printtoconsole=0 &

    # Make sure it's listening on its p2p port.
    sleep 1

    "$bitcoind" -regtest $fallback_fee_arg $block_filter_arg \
                -datadir="${test_dir}/2" \
                -connect=127.0.0.1:"${version_id}48" \
                -rpcport="$rpc_port" \
                -rpcuser="$RPC_USER" \
                -rpcpassword="$RPC_PASSWORD" \
                -server=1 \
                -txindex=1 \
                -printtoconsole=0 \
                -zmqpubrawblock=tcp://0.0.0.0:"${version_id}32" \
                -zmqpubrawtx=tcp://0.0.0.0:"${version_id}33" &

    # Let it connect to the other node.
    sleep 1

    echo "Two connected bitcoind v${version_number} instances running, one node has JSON-RPC listening on port ${rpc_port}"
}

say() {
    echo "run_bitcoind: $1"
}

err() {
    echo "$1" >&2
    exit 1
}

#
# Main script
#
main "$@"
exit 0
