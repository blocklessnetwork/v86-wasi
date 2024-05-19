#!/bin/bash
BIN_DIR="v86-wasi-bin"
function cleanup {
	echo rm -rf $BIN_DIR > /dev/null
}
function fail {
	cleanup
	msg=$1
	echo "============"
	echo "Error: $msg" 1>&2
	exit 1
}

function install_macos_tap_driver {
	echo "Install the tap driver".
	URL="https://raw.githubusercontent.com/blocklessnetwork/v86-wasi/main/third_party/macos-taptun.tar.gz"
	DRIVERS="$BIN_DIR/drivers"
	mkdir $DRIVERS -p
	curl -s $URL -o $DRIVERS/macos-tuntap.tar.gz
	cd $DRIVERS 
	tar xzvf macos-tuntap.tar.gz
	mv tunnelblick-tap.kext /Library/Extensions/
	mv tunnelblick-tun.kext /Library/Extensions/
	kextload /Library/Extensions/tunnelblick-tap.kext
	kextload /Library/Extensions/tunnelblick-tun.kext
}

function install {
	#settings
	USER="blocklessnetwork"
	PROG='v86-wasi'
	MOVE="true"
	RELEASE="latest"
	INSECURE="false"
	OUT_DIR=`pwd`
	GH="https://github.com"
	#bash check
	[ ! "$BASH_VERSION" ] && fail "Please use bash instead"
	[ ! -d $OUT_DIR ] && fail "output directory missing: $OUT_DIR"
	#dependency check, assume we are a standard POISX machine
	which find > /dev/null || fail "find not installed"
	which xargs > /dev/null || fail "xargs not installed"
	which sort > /dev/null || fail "sort not installed"
	which tail > /dev/null || fail "tail not installed"
	which cut > /dev/null || fail "cut not installed"
	which du > /dev/null || fail "du not installed"
	GET=""
	if which curl > /dev/null; then
		GET="curl"
		if [[ $INSECURE = "true" ]]; then GET="$GET --insecure"; fi
		GET="$GET --fail -# -L"
	elif which wget > /dev/null; then
		GET="wget"
		if [[ $INSECURE = "true" ]]; then GET="$GET --no-check-certificate"; fi
		GET="$GET -qO-"
	else
		fail "neither wget/curl are installed"
	fi
	#find OS #TODO BSDs and other posixs
	case `uname -s` in
	Darwin) OS="darwin";;
	Linux) OS="linux";;
	*) fail "unknown os: $(uname -s)";;
	esac
	#find ARCH
	if uname -m | grep 64 | grep arm > /dev/null; then
		ARCH="arm64"
	elif uname -m | grep 64 | grep aarch > /dev/null; then
		ARCH="arm64"
	elif uname -m | grep 64 > /dev/null; then
		ARCH="amd64"
	elif uname -m | grep arm > /dev/null; then
		ARCH="arm" #TODO armv6/v7
	elif uname -m | grep 386 > /dev/null; then
		ARCH="386"
	else
		fail "unknown arch: $(uname -m)"
	fi


	BROWER_CMD="open"
	
	if [[ $OS = "darwin" ]]; then
		BROWER_CMD="open"
	fi

	if [[ $OS = "linux" ]]; then
		BROWER_CMD="firefox"
	fi

	#choose from asset list
	URL=""
	FTYPE=""
    if [ ! -n "$VERSION" ]; then
	    VERSION="latest"
    fi
    WASM_URL="https://github.com/blocklessnetwork/v86-wasi/releases/${VERSION}/download/v86.x86_32.tar.gz"
    FTYPE=".tar.gz"
	case "${OS}_${ARCH}" in
	"darwin_amd64")
		URL="https://github.com/blocklessnetwork/v86-wasi/releases/${VERSION}/download/v86-wasi.macos-latest.x86_64.tar.gz"
		;;
	"darwin_arm64")
		URL="https://github.com/blocklessnetwork/v86-wasi/releases/${VERSION}/download/v86-wasi.macos-latest.aarch64.tar.gz"
		;;
	"linux_amd64")
		URL="https://github.com/blocklessnetwork/v86-wasi/releases/${VERSION}/download/v86-wasi.linux-latest.x86_64.tar.gz"
		;;
	"linux_arm64")
		URL="https://github.com/blocklessnetwork/v86-wasi/releases/${VERSION}/download/v86-wasi.linux-latest.aarch64.tar.gz"
		;;
	*) fail "No asset for platform ${OS}-${ARCH}";;
	esac
	#got URL! download it...
	echo -n "Installing $PROG $VERSION"
	
	echo "....."
	
	#enter tempdir
	mkdir -p $BIN_DIR
	cd $BIN_DIR
	if [[ $FTYPE = ".gz" ]]; then
		which gzip > /dev/null || fail "gzip is not installed"
		#gzipped binary
		NAME="${PROG}_${OS}_${ARCH}.gz"
		GZURL="$GH/releases/download/$RELEASE/$NAME"
		#gz download!
		bash -c "$GET $URL" | gzip -d - > $PROG || fail "download failed"
	elif [[ $FTYPE = ".tar.gz" ]] || [[ $FTYPE = ".tgz" ]]; then
		#check if archiver progs installed
		which tar > /dev/null || fail "tar is not installed"
		which gzip > /dev/null || fail "gzip is not installed"
		echo "GET BIN"
		bash -c "$GET $URL" | tar zxf - || fail "download failed"
		echo "GET WASM"
        bash -c "$GET $WASM_URL" | tar zxf - || fail "download failed"
	elif [[ $FTYPE = ".zip" ]]; then
		which unzip > /dev/null || fail "unzip is not installed"
		bash -c "$GET $URL" > tmp.zip || fail "download failed"
		unzip -o -qq tmp.zip || fail "unzip failed"
		rm tmp.zip || fail "cleanup failed"
	elif [[ $FTYPE = "" ]]; then
		bash -c "$GET $URL" > "b7s_${OS}_${ARCH}" || fail "download failed"
	else
		fail "unknown file type: $FTYPE"
	fi
	curl -s https://raw.githubusercontent.com/blocklessnetwork/v86-wasi/main/boot.json | sed 's/target\///g' > $OUT_DIR/$BIN_DIR/boot.json
	curl -s https://raw.githubusercontent.com/blocklessnetwork/v86-wasi/main/run.sh | sed 's/target\///g' > $OUT_DIR/$BIN_DIR/run.sh
	chmod +x $OUT_DIR/$BIN_DIR/run.sh

	echo "Installed $PROG $VERSION at $OUT_DIR/$BIN_DIR"
	WHOAMI=`whoami`
	if [[ "$WHOAMI" = "root" ]]; then
		if [[ $OS = "darwin" ]]; then
			install_macos_tap_driver
		fi
	fi
	$BROWER_CMD $OUT_DIR/$BIN_DIR/term/term.html
	cd $OUT_DIR/$BIN_DIR && ./run.sh
	
	#done
	cleanup
}
install $1
