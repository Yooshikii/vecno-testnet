
# **Vecno Mainnet Setup Guide (Used as Testnet)**

This guide provides step-by-step instructions for setting up a Vecno Wallet, running a Vecno Node, compiling the Vecno Miner, and mining on the Vecno Mainnet (used as Testnet). Follow these steps carefully for a smooth and secure experience.

## **Prerequisites**

* **Operating System**: Windows or Linux
* **Software**: Download the latest Vecno Wallet, Vecno Node, and miner binaries or source code from the [Vecno Foundation GitHub](https://github.com/Vecno-Foundation).
* **Hardware**: A computer with sufficient CPU or GPU for mining (optional).
* **Internet**: Stable connection for syncing with the network.
* **For Building from Source**: Install Rust and Cargo (see [rustup.rs](https://rustup.rs/)).

---

## **1. Setting Up Vecno Wallet**

### **1.1 Launch Vecno Wallet**

**Open the Vecno Wallet application:**

* **Windows**: Double-click **vecno-wallet.exe,** found in [Releases](https://github.com/Vecno-Foundation/vecno-testnet/releases).
* **Linux**: Run in a terminal:

  ```bash
  ./vecno-wallet
  ```

### **1.2 Connect to the Network**

1. **Select the Mainnet:**

   ```bash
   network mainnet
   ```
2. **Connect to a server:**

   **Option 1: Use an External Node**
   Connect to a public Vecno node:

   ```bash
   connect
   ```

   **Option 2: Use Your Own Node (Recommended)**
   If running a local Vecno node (see Section 2), connect to it:

   ```bash
   server localhost
   connect
   ```

   **Note**: If you encounter connection issues, verify the server address or check your internet connection.

### **1.3 Create a Wallet**

1. **Run the wallet creation command:**

   ```bash
   wallet create 'your-wallet-name-here'
   ```
2. **Provide the following details:**

   * **Wallet Name**: Choose a unique name for your wallet (used to open it later).
   * **Phishing Hint** (Optional): Set a secret word/phrase to verify wallet authenticity when opening.
   * **Encryption Password**: Enter a strong password to secure your wallet. Confirm by re-entering it.
3. **Optional: Add a Mnemonic Passphrase**

   * Generate a BIP39 mnemonic using the [Vecno Mnemonic Generator](https://github.com/Vecno-Foundation/mnemonic-generator).
   * **Enter the mnemonic during wallet creation for enhanced security and recovery.**
   * **Warning**: Store the mnemonic securely. Losing it prevents wallet recovery or transaction authorization.
   * If you skip this step, leave the mnemonic field blank.

**Success**: Your wallet is now created! Save your wallet name, phishing hint, password, and mnemonic (if used) in a secure location.

---

## **2. Running a Vecno Node**

Running a Vecno node enables wallet connections to a local server or mining on the Mainnet. You can start the node using a preconfigured script, a binary, or build from source.

### **2.1 Start the Node with a Script**

1. **Launch the Node**:
   * **Windows/Linux: Run example_run_vecnod.bat (Windows) or the equivalent Linux script.**
   * **This script starts the node with default settings optimized for mining. Syncing may take a few minutes.**
2. **Verify Operation**:
   * Check the terminal for sync progress or errors.
   * Ensure your firewall allows outbound connections on port 7111 (Mainnet default).

**Note**: Use this method for quick setup if you don’t need custom configurations.

### **2.2 Start the Node with Binary (Recommended for Wallet)**

To run the node with wallet support using the vecnod binary:

1. **Command**:
   * Navigate to the directory containing vecnod.
   * Run:

     ```bash
     ./vecnod --utxoindex --rpclisten-borsh
     ```
2. **Purpose of Flags**:
   * **--utxoindex**: Enables the UTXO index, required for wallet operations.
   * **--rpclisten-borsh**: Activates the Borsh-serialized RPC listener for wallet communication.
3. **Syncing**:
   * The node will sync with the Mainnet, which may take time depending on network speed and blockchain size.
   * Monitor sync progress in the terminal or logs.
4. **Troubleshooting**:
   * **Wallet Connection Issues**: Ensure **--rpclisten-borsh** is included and the RPC port (**7110** by default) is open.
   * **Sync Delays**: Use **--addpeer=`<ip>`** to connect to reliable peers or check your internet connection.
   * **High Resource Usage**: Reduce **--maxinpeers** (default: **117**) or **--rpcmaxclients** (default: **128**) if needed.

### **2.3 Build and Run from Source (Advanced)**

To build and run the node from source for the latest features:

1. **Prerequisites**:
   * See [Readme.md](README2.md)
   * Clone the Vecno node repository from the [Vecno Foundation GitHub](https://github.com/Vecno-Foundation).
2. **Build and Run**:
   * Navigate to the repository directory.
   * Run with wallet support:

     ```bash
     cargo run --release --bin vecnod -- --utxoindex --rpclisten-borsh=public
     ```
   * **Flags Explanation**:

     * **--release**: Builds an optimized binary for better performance.
     * **--bin vecnod**: Specifies the **vecnod** binary.
     * **--utxoindex**: Enables the UTXO index for wallet support.
     * **--rpclisten-borsh=public**: Exposes the Borsh RPC listener publicly for wallet connections.
3. **Syncing and Verification**:
   * The node will sync with the Mainnet. Check the terminal for progress.
   * Ensure the node is accessible on the RPC port for wallet operations.

### **2.4 Custom Arguments (Optional)**

**Customize the node with these common flags:**

* **--appdir=`<path>`**: Set the data directory (default: **/Users/`<user>`/Library/Application Support/Vecnod**).
* **--logdir=`<path>`**: Specify a log output directory.
* **--listen=<ip:port>**: Set the interface/port for incoming connections (default: **0.0.0.0:7111**).
* **--rpclisten=<ip:port>**: Set the standard RPC port (default: **7110**).
* **--addpeer=`<ip>`**: Add a peer to connect to at startup.
* **--loglevel=`<level>`**: Set logging verbosity (e.g., **debug**, **info**; default: **info**).

**Example with additional flags:**

```bash
./vecnod --utxoindex --rpclisten-borsh --logdir=/path/to/logs --loglevel=debug
```

### **2.5 Archival Node (Optional)**

**To retain all block data (requires significant disk space):**

```bash
./vecnod --utxoindex --rpclisten-borsh --archival
```

**Warning**: This mode significantly increases disk usage.

### **2.6 Mining Considerations**

* **Important**: Do not mine while syncing, as it may disrupt the blockchain. Ensure the node is fully synced before starting the miner (see Section 4).
* Monitor sync status in the terminal or logs before initiating mining.

**Success**: Your node is now running and configured for wallet operations or mining on the Vecno Mainnet!

---

## **3. Compiling Vecno Miner from Source**

To mine on the Vecno Testnet, you may need to compile the vecno-miner from source for the latest features or customizations.

1. **Prerequisites**:
   * Ensure Rust and Cargo are installed (see [rustup.rs](https://rustup.rs/)).
   * Clone the Vecno miner repository from the [Vecno-Miner GitHub](https://github.com/Vecno-Foundation/vecno-miner).
2. **Build the Miner**:
   * Navigate to the miner repository directory.
   * Compile the miner:

     ```bash
     cargo build --release --all
     ```
   * This creates an optimized vecno-miner binary in the target/release directory.
3. **Configure the Miner**:
   * Locate the compiled vecno-miner binary (e.g., target/release/vecno-miner).
   * Create or edit a script (e.g., run_vecno-miner.sh) to include your wallet address and CPU threads:

     ```bash
     ./target/release/vecno-miner --mining-address=YOUR_WALLET_ADDRESS_HERE --threads=NUMBER_OF_THREADS --port 7110
     ```
   * Replace YOUR_WALLET_ADDRESS_HERE with the wallet address from Section 1.3.
   * Adjust NUMBER_OF_THREADS based on your CPU capabilities (e.g., 4 or 8 for a standard CPU).
4. **Troubleshooting**:
   * **Build Errors**: Ensure Rust and Cargo are up to date and dependencies are installed (**cargo update**).
   * **Invalid Address**: Double-check your wallet address format.
   * **Performance Issues**: Reduce the number of threads if your system is overloaded.

---

## **4. Mining on Vecno Testnet**

### **4.1 Configure the Miner**

1. **Open the miner script:**

   * If using a prebuilt binary, locate example_run_vecno-miner.bat (Windows) or equivalent Linux script and right-click to select "Edit" (Windows) or open in a text editor (Linux).
   * If using a compiled binary, edit your custom script (e.g., run_vecno-miner.sh).
2. **Update the settings:**

   * Replace --mining-address vecno:XXXX with your wallet address from Section 1.3.
   * Adjust --threads NUMBER_OF_THREADS based on your CPU capabilities (e.g., 4–8 threads).
     Example:

   ```bash
   --mining-address vecno:YOUR_WALLET_ADDRESS_HERE --threads=6
   ```
3. **Save the changes.**

### **4.2 Begin Mining**

1. **Ensure your node is fully synced (check terminal or logs from Section 2).**
2. **Run the miner script:**
   * For prebuilt binary: Run example_run_vecno-miner.bat.
   * For compiled binary: Run your custom script (e.g., ./run_vecno-miner.sh).
3. **Monitor the terminal for mining status (e.g., hash rate, block rewards).**
4. **Troubleshooting**:
   * **Mining Fails**: Ensure the node is fully synced and the wallet address is correct.
   * **Low Performance**: Adjust **--threads** or check CPU usage.
   * **Network Issues**: Verify node connectivity with **--addpeer=`<ip>`**.

**Success**: You are now mining on the Vecno Mainnet!

---

**Tips for a Secure and Smooth Experience**

* **Security**:
  * Always verify the phishing hint when opening your wallet to avoid phishing attacks.
  * Back up your mnemonic passphrase, wallet password, and wallet name in multiple secure locations (e.g., offline storage).
  * Secure RPC keys (--rpckey, --rpccert) and avoid exposing --rpclisten-borsh=public unnecessarily.
* **Performance**:
  * Allocate sufficient CPU threads for mining but avoid overloading your system.
  * Ensure your node stays synced by keeping it running or checking sync status periodically.
* **Updates**:
  * Regularly check the [Vecno Foundation GitHub](https://github.com/Vecno-Foundation) for software updates and security patches.
* **Support**:
  * For issues, join the Vecno community on [Discord](https://discord.com/invite/Vm7rc49cWm) or consult the official documentation.
