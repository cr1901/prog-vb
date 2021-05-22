# This script takes care of building your crate and packaging it for release

set -ex

main() {
    local src=$(pwd) \
          stage= \
          suffix=

    TAG="v0.10.0"

    case $OS in
        ubuntu-latest)
            stage=$(mktemp -d)
            ;;
        macos-latest)
            stage=$(mktemp -d -t tmp)
            ;;
    esac

    case $TARGET in
        x86_64-pc-windows-gnu)
            suffix=.exe
            ;;
        *)
            suffix=
    esac

    test -f Cargo.lock || cargo generate-lockfile

    # TODO Update this to build the artifacts that matter to you
    CARGO_PROFILE_RELEASE_LTO="thin" cross build --target $TARGET --release

    mkdir $stage/$CRATE_NAME-$TAG
    cp target/$TARGET/release/prog-vb$suffix CHANGELOG.md LICENSE.md README.md $stage/$CRATE_NAME-$TAG

    cd $stage
    if [ $TARGET = x86_64-pc-windows-gnu ]; then
        # We cross-compile to Windows; convert to Windows-style endings and use zip.
        sudo apt-get install -y dos2unix
        unix2dos -s $CRATE_NAME-$TAG/*.*
        zip $src/$CRATE_NAME-$TAG-$TARGET.zip $CRATE_NAME-$TAG/*.*
    else
        tar czf $src/$CRATE_NAME-$TAG-$TARGET.tar.gz $CRATE_NAME-$TAG
    fi
    cd $src

    rm -rf $stage
}

main
