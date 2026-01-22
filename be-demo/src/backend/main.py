from fastapi import FastAPI

app = FastAPI()

EMAIL = "a@gmail.com"
PASSWORD = "r2isthebest"

@app.get("/")
async def root():
    return {"message": f"Welcome to BE!"}

@app.get("/health")
async def health_check():
    return {"status": "ok"}

@app.get("/api/login")
async def health_check():
    return {"status": "ok"}

@app.get("/api/logout")
async def health_check():
    return {"status": "ok"}

@app.get("/api/try_luck")
async def health_check():
    return {"win": True}
