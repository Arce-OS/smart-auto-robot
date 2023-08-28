import asyncio
import aiohttp
import cv2
import time
import numpy as np
import socket
async def main():
    running = False
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.connect(("192.168.1.11",6000))
    s.send(b"$010F040115#r\n")
    s.send(b"$01160610103d#r\n")
    s.send(b"$011504001A#\r\n")
    async with aiohttp.ClientSession() as session:
        async with session.get("http://192.168.1.11:6500/video_feed") as resp:
            reader = aiohttp.MultipartReader.from_response(resp)
            metadata = None
            filedata = None
            while True:
                part = await reader.next()
                if part is None:
                    break
                if part.headers[aiohttp.hdrs.CONTENT_TYPE] == 'image/jpeg':
                    data = await part.read()
                    rgb = cv2.imdecode(np.asarray(data,dtype="uint8"),cv2.IMREAD_COLOR)
                    rgb_filtered = cv2.GaussianBlur(rgb,(3,3),0)
                    hsv = cv2.cvtColor(rgb_filtered,cv2.COLOR_BGR2HSV)
                    mask = cv2.inRange(hsv,np.array([0,80,46]),np.array([5,255,255])) + cv2.inRange(hsv,np.array([175,80,46]),np.array([180,255,255]))
                    if np.mean(mask) > 64:
                        if not running:
                            s.send(b"$01100800200039#\r\n")
                            running = True
                            print("run")
                    else:
                        if running:
                            s.send(b"$0115040721#\r\n")
                            running = False
                            print("stop")
                        
asyncio.run(main())
    