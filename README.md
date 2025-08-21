# Clync - Clipboard Sync

Clync is a simple, fast, and cross-platform tool to synchronize your clipboard across multiple devices on your local network.

## How It Works

Clync uses a client-server model. One machine runs as a central **server**, and all your other devices (including the server machine itself) run a **client**. The clients send their clipboard updates to the server, which then distributes them to all other connected clients.

## Installation

_Instructions for building from source or using pre-compiled binaries would go here._

## Setup and Usage

To get clipboard syncing working between your devices (e.g., a Windows PC and a Mac), you need to run three processes in total: one server and two clients.

### Step 1: Choose Your Server Machine

First, decide which computer will act as the central server. It's usually best to choose a computer that is always on, like a desktop PC. In this guide, we will assume the **Windows PC** is the server.

### Step 2: Find the Server's IP Address

You will need the server's local IP address so the other devices know where to connect.

**On Windows:**
1. Open terminal.
2. Type the command `ipconfig` and press Enter.
3. Look for the "IPv4 Address" under your active network connection (e.g., "Wireless LAN adapter Wi-Fi"). It will likely look something like `192.168.1.X`.
4. Take note of this IP address. For this guide, we'll use `192.168.1.21` as an example (You should use your own IP address).

**On macOS or Linux:**
1. Open the Terminal.
2. Type the command `ifconfig | grep "inet "` and press Enter.
3. Look for the address that is not `127.0.0.1`. It will likely start with `192.168.X.X` or `10.0.X.X`.

### Step 3: Configure Your Firewall (Important!)

Operating systems like Windows have a firewall that blocks incoming network connections by default. You must create a rule to allow Clync to work.

**On Windows (Server Machine):**
1. Open **Windows Defender Firewall with Advanced Security**.
2. Go to **Inbound Rules** > **New Rule...**.
3. Select **Port** and click Next.
4. Select **TCP**, then **Specific local ports**, and enter `8080`. Click Next.
5. Select **Allow the connection** and click Next.
6. Ensure the **Private** network profile is checked. Click Next.
7. Give the rule a name, like `Clync Server`, and click Finish.

### Step 4: Run the Three Processes

You are now ready to start the application.

**1. Start the Server (On your Windows PC)**
   - Open a new terminal or Command Prompt.
   - Run the following command to start the server. This terminal must be left open.
   ```bash
   clync server 8080
   ```

**2. Start the Client (On your Windows PC)**
   - Open a **second, new** terminal or Command Prompt.
   - Run the client, telling it to connect to the server on the same machine (`localhost`).
   ```bash
   clync client http://localhost:8080
   ```

**3. Start the Client (On your Mac or other device)**
   - Open a terminal.
   - Run the client, pointing it to the IP address of your Windows server that you found in Step 2. (Using 192.168.1.21 as an example, kindly use your own IP address)
   ```bash
   clync client http://192.168.1.21:8080
   ```

### You're Done!

Your clipboards are now synced. You can copy text on any of the client machines, and it will be available to paste on all the others.

## Troubleshooting

- **"Network error sending to server"**: This means a client cannot reach the server.
  - **Check the IP Address**: Make sure you are using the correct IP for the server machine.
  - **Check the Firewall**: This is the most common cause. Ensure the firewall rule on the server machine is correct and enabled for your current network profile (Private vs. Public).
  - **Ping the Server**: From a client machine, try to `ping <SERVER_IP>`. If it fails, there is a basic network connectivity issue that needs to be resolved.
