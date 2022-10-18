## Dattas Pallet Overview

    Dattas is an example module for keeping track of account names on-chain. It makes no effort to
    create a name hierarchy, be a DNS replacement or provide reverse lookups.

## Dattas Functions

- `set_name` - Set the associated name of an account; a small deposit is reserved if not already taken.
- `clear_name` - Remove an account's associated name; the deposit is returned.
- `kill_name` - Forcibly remove the associated name; the deposit is lost.
- `balanceOf` - get the balance of the user.
- `ReservedNameOf` - get the associated name with reserved deposite.

## Prerequisites - Installation steps

    Before we clone the repo and start with the execution, we need to make sure our system is compatible and have all the required prerequisites installed.

## Assumption -

    I assume you are running on the Mac operating system and below steps are according to macOs, For other OS installation steps may differ!

1.  Please make sure you have homebrew installed on you machine, if not - please follow below steps
    a) Open the Terminal application.

    b) Download and install Homebrew by running the following command:
    /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install.sh)"

    c) check version - brew --version

2.  We need openssl because the blockchain requires standard cryptography to support the generation of public/private key pairs and the validation of transaction signatures, you must also have a package that provides cryptography, such as openssl. If openssl is not installed, please follow below steps
    a) brew update

    b) Install the openssl package by running the following command
    brew install openssl

3.  Please make sure you have Rust installed, if not - please follow below steps
    a) Download the rustup installation program and use it to install Rust by running the following command -
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

    b) Follow the prompts displayed to proceed with a default installation.

    c) Update your current shell to include Cargo by running the following command -
    source ~/.cargo/env

    d) Verify your installation by running the following command -
    rustc --version

    d) Configure the Rust toolchain to default to the latest stable version by running the following commands -
    rustup default stable
    rustup update

    e) Add the nightly release and the nightly WebAssembly (wasm) targets to your development environment -
    rustup update nightly
    rustup target add wasm32-unknown-unknown --toolchain nightly

    f) Verify the configuration of your development environment by running the following command -
    rustup show
    rustup +nightly show

    g) The above command displays output similar to the following -
    Default host: aarch64-apple-darwin
    rustup home: /Users/dattajadhav44/.rustup

             installed toolchains
             --------------------

             stable-aarch64-apple-darwin (default)
             nightly-2021-06-16-aarch64-apple-darwin
             nightly-aarch64-apple-darwin

             active toolchain
             ----------------

             stable-aarch64-apple-darwin (default)
             rustc 1.64.0 (a55dd71d5 2022-09-19)

             Default host: aarch64-apple-darwin
             rustup home:  /Users/dattajadhav44/.rustup

             installed toolchains
             --------------------

             stable-aarch64-apple-darwin (default)
             nightly-2021-06-16-aarch64-apple-darwin
             nightly-aarch64-apple-darwin

             installed targets for active toolchain
             --------------------------------------

             aarch64-apple-darwin
             wasm32-unknown-unknown

             active toolchain
             ----------------

             nightly-aarch64-apple-darwin (overridden by +toolchain on the command line)
             rustc 1.66.0-nightly (81f391930 2022-10-09)

    h) Install cmake using the following command:
    brew install cmake

4.  Please make sure you have Git installed.

5.  Please make sure you have node, npm/yarn installed on your system. These are generic packages and not only related to rust or substrate.

6.  Well done - So we have got the development machine ready for the execution :)

# -----------------------------------------------------------------------------------------------------------------

## Execution steps -

    There two things [a] node which has our pallet changes as a part of this assigment [b] Front-end template from the substrate. Let us start with node changes and then front-end

1.  Firstly, let's clone the repo -
    a) Open the Terminal application. Clone the repo by running the following command -
    git clone https://github.com/DattatrayJadhav/substrate-exercise.git

    b) Navigate to the source directory by running the following command -
    cd substrate-exercise

    c) Compile the substrate-exercise by running the following command -
    cargo build --release [ please use the --release flag always to build optimized artifacts]

    d) Now, we need start the node - Start the node in development mode by running the following command -
    ./target/release/node-template --dev [ --dev flag denotes we are running in development environment ]

    e) Verify your node is up and running successfully by reviewing the output displayed in the terminal
    The terminal should display output similar to this -
    2022-10-18 15:30:41 Substrate Node  
     2022-10-18 15:30:41 ✌️ version 4.0.0-dev-unknown  
     2022-10-18 15:30:41 ❤️ by Substrate DevHub <https://github.com/substrate-developer-hub>, 2017-2022  
     2022-10-18 15:30:41 📋 Chain specification: Development  
     2022-10-18 15:30:41 🏷 Node name: paltry-respect-1621  
     2022-10-18 15:30:41 👤 Role: AUTHORITY  
     2022-10-18 15:30:41 💾 Database: RocksDb at /var/folders/1w/jvjgtw2s5l93nyv024m3sldr0000gn/T/substrate9xqKie/chains/dev/db/full  
     2022-10-18 15:30:41 ⛓ Native runtime: node-template-100 (node-template-1.tx1.au1)  
     2022-10-18 15:30:41 🔨 Initializing Genesis block/state (state: 0xa5ea…7067, header-hash: 0x65e7…b83b)  
     2022-10-18 15:30:41 👴 Loading GRANDPA authority set from genesis on what appears to be first startup.  
     2022-10-18 15:30:41 Using default protocol ID "sup" because none is configured in the chain specs  
     2022-10-18 15:30:41 🏷 Local node identity is: 12D3KooWLoD4EddoAiJFYWoGDcFMv6Rwu6hPsEFn4Dm9ZZTezHUC  
     2022-10-18 15:30:41 💻 Operating system: macos  
     2022-10-18 15:30:41 💻 CPU architecture: aarch64  
     2022-10-18 15:30:41 📦 Highest known block at #0  
     2022-10-18 15:30:41 〽️ Prometheus exporter started at 127.0.0.1:9615  
     2022-10-18 15:30:41 Running JSON-RPC HTTP server: addr=127.0.0.1:9933, allowed origins=None  
     2022-10-18 15:30:41 Running JSON-RPC WS server: addr=127.0.0.1:9944, allowed origins=None  
     2022-10-18 15:30:42 🙌 Starting consensus session on top of parent 0x65e7354a50610f25385730e560b2f71014ff7af4d10223bba84a63dfe227b83b  
     2022-10-18 15:30:42 🎁 Prepared block for proposing at 1 (2 ms) [hash: 0x000c30be4f5fe0ca880c8a11321094f3427c5d790989248be40fb126c9fa77de; parent_hash: 0x65e7…b83b; extrinsics (1): [0x1e85…4e53]]  
     2022-10-18 15:30:42 🔖 Pre-sealed block for proposal at 1. Hash now 0xdd1eba663af9a43b1e3f2c5290b26a3dd29aaf28ad539f98010e1b4cc15949be, previously 0x000c30be4f5fe0ca880c8a11321094f3427c5d790989248be40fb126c9fa77de.  
     2022-10-18 15:30:42 ✨ Imported #1 (0xdd1e…49be)  
     2022-10-18 15:30:43 Accepting new connection, 1/100
    2022-10-18 15:30:43 Accepting new connection, 2/100

2.  Second, lets clone the Front-end repo which is provided by the substrate doc
    a) Open the another Terminal application. Clone the front-end by running the following command -
    git clone https://github.com/substrate-developer-hub/substrate-front-end-template

    b) Navigate to the source directory by running the following command:
    cd substrate-front-end-template

    c) Install the dependencies for the front-end template by running the following command -
    yarn install

    d) Start the front-end template by running the following command -
    yarn start

    e) Open http://localhost:8000 in a browser to view the front-end

3.  On the http://localhost:8000 page go down to Pallet Interactor
    a) In the Pallet Interactor component select that Extrinsic.

    b) You should see "dattas" pallet there in the list. Please select that.

    c) Please select the "setName" method

    d) In name - Type a name that is longer than the MinLength (8 chars) and no longer than the MaxLength (32 chars).
    example - substrate learning - Datta

    e) Click Signed to execute the function.

    f) Observe the status of the call change from Ready to InBlock to Finalized and the note the events emitted by the Dattas pallet at right side of the page.
    You should see something like below.
    😉 Finalized. Block hash: 0x5bdf60f725929a62776f9264665b7f7e92906e9b7c9c77b037c02f0a237377df
    Events
    system:ExtrinsicSuccess
    [{"weight":"50,000,000","class":"Normal","paysFee":"Yes"}]
    dattas:NameSet
    ["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"]
    balances:Reserved
    ["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","500"]
    balances:Withdraw
    ["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","86,298,134"]
    system:ExtrinsicSuccess
    [{"weight":"70,000,000","class":"Normal","paysFee":"Yes"}]

    g) In the Pallet Interactor component, select Query as the Interaction Type.

    h) Select "dattas" from the list of pallets available to query.

    i) Copy and paste the address for the alice account in the AccountId field, then click Query.
    You should the result something like below
    ["0x537562737472617465206c6561726e696e67202d204461747461",500]

    j) Similarly you can try out other couple of methods like [clearName, killName, forceName, balanceOf]

4.  Please see the attached snippets in the test-snippet folder.
