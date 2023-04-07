import asyncio
import json

async def main():
    """connect to port 4001 and send a message"""
    reader, writer = await asyncio.open_connection("localhost", 4001)
    writer.write(json.dumps({"Inbound": "RegisterUI"}).encode())
    # writer.write(json.dumps({"command": "RegisterClient", "event": "Inbound"}).encode())
    # writer.write(b"hello")
    await writer.drain()
    writer.close()
    await writer.wait_closed()
    data = await reader.read(100)


if __name__ == '__main__':
    loop = asyncio.new_event_loop()
    loop.run_until_complete(main())
