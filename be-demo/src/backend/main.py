from fastapi import FastAPI, HTTPException
from starlette.types import HTTPExceptionHandler

from backend.models import LoginResponse, LoginRequest
from backend.utils import is_valid_email, validate_password

app = FastAPI()

@app.get("/health")
async def health_check():
    return {"status": "ok"}

@app.post("/api/login", response_model=LoginResponse)
async def login(payload: LoginRequest):
    try:
        normalized_email = is_valid_email(payload.email)
        if not validate_password(normalized_email, payload.password):
            raise HTTPException(status_code=401, detail="Invalid password")
        return LoginResponse(token="MOCK_TOKEN")
    except Exception as e:
        raise HTTPException(status_code=400, detail=str(e))

@app.get("/api/logout")
async def health_check():
    return {"status": "ok"}

@app.get("/api/try_luck")
async def health_check():
    return {"win": True}
