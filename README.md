# Substrate & FRAME Supply Chain Sample

This sample project demonstrates how the [Substrate](https://www.substrate.io/) framework for
building blockchains and its standard
[FRAME](https://substrate.dev/docs/en/knowledgebase/runtime/frame) library for runtime development
can be used to quickly build an ergonomic, end-to-end, blockchain-based application. This sample
includes a custom front-end that was created from the helpful
[Substrate Front-End Template](https://github.com/substrate-developer-hub/substrate-front-end-template),
which itself makes use of the powerful [Polkadot{JS}](https://polkadot.js.org/) API. The
[chain](chain) in this sample is a fork of the official
[Substrate Node Template](https://github.com/substrate-developer-hub/substrate-node-template) - a
supported starting point that decreases the time it takes to implement a custom next-generation
blockchain.

The use case that this sample demonstrates is a decentralized supply-chain consortium. In order to
accomplish this, FRAME is used to implement custom business logic as well as existing standards,
such as [decentralized identifiers (DIDs)](https://w3c.github.io/did-core/).

The capabilities demonstrated by this project include:

- Fine-grained and performance-preserving role-based access control (RBAC).
- Set-up and coordinate a decentralized network (permissioned blockchain) among several
  organisations.
- Manage decentralized identities for member organisations and their delegates.
- Register products and associated metadata, such as the organisation that owns them.
- Create shipments and track their journey through the supply chain.
- Monitor a shipment's storage and transportation conditions.

The sample demonstrates many features and capabilities of the
[Substrate framework](https://substrate.dev/), including:

- Consortium network with a
  [proof-of-authority consensus](https://en.wikipedia.org/wiki/Proof_of_authority)
  ([Aura](https://substrate.dev/docs/en/knowledgebase/advanced/consensus#aura) for block production,
  [GRANDPA](https://substrate.dev/docs/en/knowledgebase/advanced/consensus#grandpa) for block
  finalization). Substrate and FRAME ship with a number of well-research and battle-tested
  [consensus mechanisms](https://substrate.dev/docs/en/knowledgebase/advanced/consensus#consensus-in-substrate)
  and also make it possible to design and implement custom consensus mechanisms.
- Dynamic set of
  [authority](https://substrate.dev/docs/en/knowledgebase/getting-started/glossary#authority) nodes.
- Role-based access control (RBAC) built on
  [signed extensions](https://substrate.dev/docs/en/knowledgebase/learn-substrate/extrinsics#signed-extension).
- Reliable real-world data integration with
  [off-chain workers](https://substrate.dev/docs/en/knowledgebase/runtime/off-chain-workers).
- Flexible blockchain runtime development that uses FRAME pallets to encapsulate domain-specific
  logic (e.g. separate pallets for product [registry](chain/pallets/registrar) &
  [tracking](https://github.com/stiiifff/pallet-product-tracking)).

## Running the Demo

Follow the [installation instructions](https://substrate.dev/docs/en/knowledgebase/getting-started/)
for getting started with Rust and Substrate. This project is built on Substrate v2.0.0, which means
that it uses the
[Rust `nightly-2020-10-05` toolchain](https://substrate.dev/docs/en/knowledgebase/getting-started/#rust-nightly-toolchain)
for Wasm compilation. Ensure that these commands are executed as part of the installation process:
process:

```bash
rustup install nightly-2020-10-05
rustup target add wasm32-unknown-unknown --toolchain nightly-2020-10-05
```

This demo also uses [Node.js](https://nodejs.org/en/) and [Yarn](https://classic.yarnpkg.com/en/),
so ensure that those dependencies are installed before continuing.

- Run the [listener](ocw-listener) that receives off-chain worker notifications

  ```bash
  cd ocw-listener
  yarn install && yarn start
  ```

- Run the [Substrate chain](chain)

  ```bash
  cd chain
  WASM_BUILD_TOOLCHAIN=nightly-2020-10-05 cargo build --release
  # Launch the node in development mode and do not persist chain state
  ./target/release/enterprise-sample --dev --tmp
  ```

- Launch the [front-end](ui)

  ```bash
  cd ui
  yarn install && yarn start
  ```

- \[Optional\] Run the initialization script to bootstrap a consortium, create organizations,
  products, and shipments, and track the shipments from creation to delivery. Continue to the
  [guided demo](#guided-demo) to perform these steps manually, and learn more about them in the
  process. The initialization script is designed to work on a newly launched chain.

  ```bash
  cd scripts
  yarn && yarn start
  ```

## Guided Demo

This guided demo will walk through a simplified version of the steps performed by the
[initialization script](./scripts/src/init.js). The demo makes use of a number of
[well-known development accounts](https://substrate.dev/docs/en/knowledgebase/integrate/subkey#well-known-keys).
In order to understand the demo's steps, it's necessary to understand the runtime modules (pallets)
that inform the supply chain application and how they relate to each other.

### Project Components

The supply chain consortium application is comprised of a number of a modules, many of which are
configured in the [chain specification](chain/node/src/chain_spec.rs)'s `development_config`
function:

- [Role-Based Access Control (RBAC) pallet](https://github.com/gautamdhameja/substrate-rbac/tree/enterprise-sample) -
  This pallet maintains an on-chain registry of roles and the users to which those roles are
  assigned. A `Role` is a tuple that encapsulates the name of a pallet and a `Permission` that
  qualifies the level of access granted by the `Role`. A `Permission` is an enum with the following
  variants: `Execute` and `Manage`. The `Execute` permission allows a user to invoke a pallet's
  [dispatchable functions](https://substrate.dev/docs/en/knowledgebase/getting-started/glossary#dispatch).
  The `Manage` permission allows a user to assign and revoke roles for a pallet, and also implies
  the `Execute` permission. Access control validation is done at the
  [transaction pool](https://substrate.dev/docs/en/knowledgebase/learn-substrate/tx-pool) validation
  layer by way of the RBAC pallet's `Authorize`
  [signed extension](https://substrate.dev/docs/en/knowledgebase/learn-substrate/extrinsics#signed-extension).
  Notice the permissions that are configured in the chain specification file. Alice is granted the
  `Execute` permission on the RBAC pallet, which allows her to use the RBAC pallet to create roles.
  In order to enable her to bootstrap the consortium, Alice is also granted the `Manage` permission
  on a few other pallet.
- [Registrar pallet](chain/pallets/registrar/src/lib.rs) - The Registrar pallet inherits
  decentralized identifier (DID) capabilities from the
  [DID pallet](https://github.com/substrate-developer-hub/pallet-did) and uses these capabilities to
  implement an organization registry. This pallet maintains a list of organizations and maps each
  organization to a list of members. Organizations are identified by the ID of the account that
  created and owns it, which means that an account may create and own _at most_ one organization.
  Organizations are associated with a name, which is designated by the value of the `Org` attribute
  on the DID of the organization owner. Organization owners are the only accounts that may add
  members to their organizations. When an account is added to an organization as a member, the
  organization owner creates an `OrgMember` delegate for the member's DID - this is a way for the
  organization owner to certify an account's membership in the organization. The registrar pallet
  exposes a custom [origin](https://substrate.dev/docs/en/knowledgebase/runtime/origin),
  `EnsureOrg`, that validates whether or not an account owns or is a member of at least one
  organization. The `EnsureOrg` origin is used to control access to many of the chain's
  capabilities, including the ability to create roles with the RBAC pallet.
- [Product Registry pallet](chain/pallets/product-registry/src/lib.rs) - This pallet maintains a
  registry of products and maps each product to the organization to which it belongs. A product is
  defined by three required properties (an ID, an owner, and a time of creation), and may have one
  or more optional user-defined properties. The `EnsureOrg` origin is used to control the accounts
  that are allowed to create products.
- [Product Tracking pallet](chain/pallets/product-tracking/src/lib.rs) - The Product Tracking pallet
  tracks shipments of products as they move throughout the supply chain. The `EnsureOrg` origin is
  used to control the accounts that are allowed to interact with this pallet. Shipments, like
  products, are assigned an ID and associated with an organization. This pallet supports tracking
  several types of shipping events: registration, pickup, scan, and delivery. With the exception of
  registration, shipment events may be associated with a list of sensor readings. Shipment events
  are placed in a queue that is monitored by an
  [off-chain worker](https://substrate.dev/docs/en/knowledgebase/runtime/off-chain-workers); when
  events appear in this queue the off-chain worker sends them to an HTTP listener.

### Demo Steps

1. Navigate to the locally deployed instance of the demo UI, which should be running at
   http://localhost:8000/demo. Notice that the UI has an account selector that default to the Alice
   account. This is important due to the special permissions that were configured for Alice in the
   chain specification file.

   ![account selector](assets/img/demo/01-accounts.png)

1. Use the Members tab to create the Execute permission for three pallets: `registrar`,
   `productRegistry`, and `productTracking`.

   ![create roles](assets/img/demo/02-create-roles.png)

1. Assign the three newly created roles to Bob, whose address is
   5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty. This will allow Bob to create an organization,
   products, shipments, and shipment events.

   ![assign roles](assets/img/demo/03-assign-roles.png)

   Note: use this link to navigate to the Polkadot{JS} Apps UI and configure it to connect to the
   local supply chain network:
   [https://polkadot.js.org/apps/#/explorer?rpc=ws://127.0.0.1:9944](https://polkadot.js.org/apps/#/explorer?rpc=ws://127.0.0.1:9944).
   Use the block hash provided by the supply chain UI to inspect the block that contained a
   transaction.

   ![Polkadot{JS} Apps UI](assets/img/demo/04-apps-ui.png)

1. Use the account selector to switch to Bob's account, and then go back to the Organizations tab to
   create an organization.

   ![create org](assets/img/demo/05-create-org.png)

1. Navigate to the Products tab and create two products.

   ![create products](assets/img/demo/06-create-products.png)

1. Use the Shipments tab to create a shipment with the two products that were created in the
   previous step.

   ![create shipment](assets/img/demo/07-create-shipment.png)

1. Navigate to the Tracking tab and use its UI to pickup, scan, and deliver the package. Note: don't
   try to include a sensor reading in a Scan or Deliver event due to an unresolved bug in this
   project.

   ![pickup shipment](assets/img/demo/08-pickup-shipment.png)

   ![scan shipment](assets/img/demo/09-scan-shipment.png)

## Related Github Repositories

- [Validator Set Pallet](https://github.com/gautamdhameja/substrate-validator-set)

## Disclaimer

This project is intended for demonstration purposes and is not audited or ready for production use.

# Substrate Connect

Run Wasm Light Clients of any Substrate based chain directly in your browser.

**Substrate Connect** is not the name of a single product, it rather describes a vision or notion that will allow developers to quickly generate and run Wasm Light Clients of their Substrate based chains - as easy as installing a node module.

Substrate Connect provides the infrastructure to run these clients directly in the browser and any other JavaScript or Node environment without deeper additional programming efforts needed. It adds Substrate light-client functionality to any Javascript environment, from in-browser applications to browser extensions and electron apps up to IOT devices and mobile phones.

It also provides an interface that enables Dapp developers to effortlessly make use of the light-client functionality in their applications.

### **Multiple building blocks on different levels are necessary to achieve this:**

1. **Ready-to-use Substrate Wasm Light-Clients** to be executed in the browser. They are part of the Substrate framework and with that, available for every Substrate based project. If developers want to generate a light client of their chain, all it takes is just one command to compile a library that contains everything that's needed to run a light client in the browser.

2) A **node module that bundles the light-clients** of different chains. It provides an interface that allows developers to run light nodes of different chains and to add runtimes and genesis configs of their own chain.

The `@substrate/connect` node module will allow developers to include light client functionality into their application by using a predefined interface.

3. For in-browser use, Substrate Connect provides a **Browser Extension** built upon the @substrate/light node module that is running the selected light clients inside the extension so that the end-user doesn't need to fire up a light node in every browser tab. This will also allow the light-node to keep syncing as long as the browser window stays open.

When used in individual projects, the Substrate Connect node module will first check for the installed extension. If available, it will try to connect to the light client running inside the extension. Only if the extension is not installed it will start a light client in the browser tab.


## Installation:

This repository is using [yarn workspaces](https://classic.yarnpkg.com/en/docs/workspaces/) for dependency management together with [Lerna](https://lerna.js.org/) to handle releases.


1. Clone the whole `substrate-connect` repository.

```bash
git clone https://github.com/paritytech/substrate-connect.git
```

2. Install all dependencies

```bash
yarn install
```

3. Compile all packages and projects

```bash
yarn build
```

To clean up all workspaces in the repository, run:

```bash
yarn clean
```

## Run local version of Burnr wallet
Running the following command will build all necessary dependencies and run the Substrate Burnr Wallet in development mode with hot reloading enabled. It will be served on http://localhost:1234/

```bash
yarn dev:burnr
```

(Make sure to run `$ yarn install` before.)

## Run local version of the Smoldot browser demo
Running the following command will build all necessary dependencies and run the Smoldot browser demo. It will be served on https://localhost:1234/

```bash
yarn dev:smoldot-browser-demo
```

(Make sure to run `$ yarn install` before.)

## Run local version of Smoldot Extension
Running the following command will build all necessary dependencies and run the Smoldot Extension in development mode with hot reloading enabled. Once run a new (Chrome) browser will appear with the extension added.

```bash
yarn dev:smoldot-extension
```

(Make sure to run `$ yarn install` before.)

## Deploy Smoldot browser demo to Github Pages

Before deploying make sure you have a clean working copy with no staged changes.
The deploy script will deploy the last commit on your current branch.

The deployment will build the smoldot browser demo into the dist folder and 
construct a commit containing just that folder with a message containing a 
reference to the SHA of the commit it came from and push that to the gh-pages
branch. The dist folder remains ignored by git.

You can deploy to Github pages like so:

```bash
yarn deploy:gh-pages:smoldot-browser-demo
```

## Deploy Smoldot browser demo to IPFS

Before deploying make sure you have a Piñata API key and secret and that you
have exported them in your shell environment:

```bash
PINATA_API_KEY=<your key>
PINATA_API_SECRET=<your secret>
```

You can then deploy to IPFS like so:

```bash
yarn deploy:ipfs:smoldot-browser-demo
```

## Working with this repository

Substrate Connect is using Yarn workspaces to manage dependencies. 

Read more about it here: https://classic.yarnpkg.com/en/docs/workspaces/

### Adding modules to single repositories

To add new dependencies, please use the following syntax:

```
yarn workspace [module name from package.json] add your-desired-npm-package
```
Example to add Jest to the Burnr Wallet:
```
yarn workspace @substrate/burnr add jest
```

Also see https://classic.yarnpkg.com/en/docs/cli/workspace/

### GH Pages Links
This is a test for GH-Pages:
[Burnr](https://paritytech.github.io/substrate-connect/burnr) and
[Smoldot Browser Demo](https://paritytech.github.io/substrate-connect/smoldot-browser-demo) and
[Multiple Network Demo](https://paritytech.github.io/substrate-connect/multiple-network-demo)
