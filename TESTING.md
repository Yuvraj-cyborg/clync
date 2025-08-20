# Testing Guide for Clync

This guide will help you test the clipboard synchronization functionality on both macOS and Windows.

## Prerequisites

- Rust installed (for building from source)
- Network connectivity between machines
- Administrative privileges may be needed for clipboard access

## Quick Test (Automated Server Testing)

Run the automated test script to verify server functionality:

```bash
./test_server.sh
```

This will test all HTTP endpoints and verify data storage/retrieval.

## Manual Testing

### 1. Testing Server Locally

Start the server on your local machine:

```bash
# From source (macOS/Linux)
cargo run --release -- server 8080

# Or using the built executable
./target/release/clync server 8080

# Windows
clync.exe server 8080
```

You should see output indicating the server is running. Test the endpoints:

```bash
# Check server status
curl http://localhost:8080/get

# Send test data
curl -X POST http://localhost:8080/sync \
     -H "Content-Type: application/json" \
     -d '{"content": "Test clipboard content"}'

# Retrieve data
curl http://localhost:8080/get
```

### 2. Testing Client Locally

In a separate terminal, start the client:

```bash
# From source (macOS/Linux)
cargo run --release -- client http://localhost:8080

# Windows
clync.exe client http://localhost:8080
```

The client should:
1. Connect to the server
2. Read your current clipboard content
3. Send it to the server every 10 seconds
4. Print "Clipboard synced successfully!" on success

### 3. Cross-Platform Testing

#### Setup A: macOS Server + Windows Client

1. **On macOS (Server)**:
   ```bash
   cargo run --release -- server 8080
   ```
   
2. **Find your macOS IP address**:
   ```bash
   ifconfig | grep "inet " | grep -v 127.0.0.1
   ```
   
3. **On Windows (Client)**:
   ```cmd
   clync.exe client http://YOUR_MACOS_IP:8080
   ```

#### Setup B: Windows Server + macOS Client

1. **On Windows (Server)**:
   ```cmd
   clync.exe server 8080
   ```
   
2. **Find Windows IP address**:
   ```cmd
   ipconfig
   ```
   
3. **On macOS (Client)**:
   ```bash
   cargo run --release -- client http://WINDOWS_IP:8080
   ```

## Testing Scenarios

### Basic Functionality Test

1. Start server on one machine
2. Start client on another machine
3. Copy text on the client machine
4. Wait 10 seconds (or check server logs)
5. Verify the content was sent to server using:
   ```bash
   curl http://SERVER_IP:8080/get
   ```

### Multi-Client Test

1. Start one server
2. Start multiple clients from different machines
3. Copy different content on each client
4. Verify that each client's clipboard data reaches the server

### Network Issues Test

1. Start server and client
2. Disconnect network temporarily
3. Copy content during disconnection
4. Reconnect network
5. Verify client reconnects and syncs

## Troubleshooting

### Server Issues

**Server won't start:**
- Check if port 8080 is already in use: `lsof -i :8080` (macOS/Linux) or `netstat -an | findstr :8080` (Windows)
- Try a different port: `clync server 8081`
- Check firewall settings

**Server starts but clients can't connect:**
- Verify IP address is correct
- Check firewall allows incoming connections on port 8080
- Test with `telnet SERVER_IP 8080`

### Client Issues

**Client can't connect:**
- Verify server is running: `curl http://SERVER_IP:8080/get`
- Check network connectivity: `ping SERVER_IP`
- Verify URL format: `http://SERVER_IP:8080` (not `https://`)

**Clipboard not syncing:**
- Check client logs for error messages
- Verify clipboard permissions (especially on macOS/Linux)
- Test manually: copy text, wait 10 seconds, check server with curl

**Permission errors:**
- On macOS: Grant terminal app accessibility permissions in System Preferences
- On Linux: Install required clipboard packages (`xclip` or `wl-clipboard`)
- On Windows: Run as administrator if needed

### Network Testing

**Test connectivity:**
```bash
# Test if server is reachable
curl -v http://SERVER_IP:8080/get

# Test from Windows
# Use PowerShell:
Invoke-WebRequest -Uri "http://SERVER_IP:8080/get"
```

**Common network issues:**
- Firewall blocking connections
- Wrong IP address (use internal network IP, not localhost)
- Server not bound to correct interface (clync binds to 0.0.0.0 by default)

## Expected Behavior

### Server
- Starts up and binds to specified port
- Accepts POST requests at `/sync` endpoint
- Accepts GET requests at `/get` endpoint
- Stores latest clipboard content from any client
- Returns stored content to any client requesting it

### Client
- Connects to server on startup
- Reads clipboard content every 10 seconds
- Sends clipboard content to server via POST
- Prints success/error messages
- Continues running until manually stopped

## Performance Testing

For high-frequency testing, you can modify the sync interval:

1. Edit `client.rs`
2. Change `Duration::from_secs(10)` to a shorter interval
3. Rebuild: `cargo build --release`

**Note:** Very short intervals may cause high CPU usage and network traffic.

## Security Considerations

- Server accepts connections from any IP (0.0.0.0)
- No authentication or encryption
- Clipboard data is stored in memory only
- Suitable for trusted networks only

For production use, consider:
- Adding HTTPS/TLS encryption
- Implementing authentication
- Adding rate limiting
- Binding to specific network interfaces only

## Logs and Debugging

Enable verbose logging:
```bash
RUST_LOG=debug cargo run -- server 8080
RUST_LOG=debug cargo run -- client http://localhost:8080
```

Check what's being sent/received:
```bash
# Monitor HTTP traffic
tcpdump -i any -A host SERVER_IP and port 8080

# On Windows, use Wireshark or similar tools
```

## Build Verification

Verify your builds are working:

```bash
# Test macOS build
./target/release/clync --help

# Test Windows build (if cross-compiled)
wine target/x86_64-pc-windows-gnu/release/clync.exe --help
```

## Success Criteria

Your clync setup is working correctly when:

1. ✅ Server starts without errors
2. ✅ Client connects to server successfully  
3. ✅ Clipboard content syncs within 10 seconds
4. ✅ Multiple clients can connect simultaneously
5. ✅ Cross-platform connectivity works (macOS ↔ Windows)
6. ✅ Server survives client disconnections
7. ✅ Client reconnects after network issues

If all tests pass, your clipboard synchronization tool is ready for use!