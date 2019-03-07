# This script takes care of building your crate and packaging it for release

set -ex

main() {
    local src=$(pwd) \
          stage= \
          suffix=

    case $TRAVIS_OS_NAME in
        linux)
            stage=$(mktemp -d)
            ;;
        osx)
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
    cross rustc --bin prog-vb --target $TARGET --release -- -C lto

    mkdir $stage/$CRATE_NAME-$TRAVIS_TAG
    cp target/$TARGET/release/prog-vb$suffix CHANGELOG.md LICENSE.md README.md $stage/$CRATE_NAME-$TRAVIS_TAG

    cd $stage
    if [ $TARGET = x86_64-pc-windows-gnu ]; then
        unix2dos -s $CRATE_NAME-$TRAVIS_TAG/*.*
        zip $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.zip $CRATE_NAME-$TRAVIS_TAG/*.*
    else
        tar czf $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.tar.gz $CRATE_NAME-$TRAVIS_TAG
    fi
    cd $src

    rm -rf $stage
}

main
