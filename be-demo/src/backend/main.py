from email_validator import EmailNotValidError
from fastapi import FastAPI, HTTPException
from starlette.types import HTTPExceptionHandler

from backend.models import LoginResponse, LoginRequest
from backend.persistence import Persistence
from backend.services import validate_email, validate_password, generate_auth_token


persistence = Persistence()

app = FastAPI()

@app.get("/health")
async def health_check():
    return {"status": "ok"}

@app.post("/api/login", response_model=LoginResponse)
async def login(payload: LoginRequest):
    """
    Attempts login; returns token or raises HTTP error
    :raises HTTPException:
    :param payload:
    :type payload: LoginRequest
    :return:
    """
    try:
        try:
            normalized_email = validate_email(payload.email)
        except EmailNotValidError as e:
            raise HTTPException(status_code=400, detail=str(e))

        if not validate_password(normalized_email, payload.password):
            raise HTTPException(status_code=401, detail="Invalid password")

        token = generate_auth_token()
        persistence.add_token(token)
        return LoginResponse(token=token)

    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

@app.get("/api/logout")
async def logout():
    return {"status": "ok"}

@app.get("/api/try_luck")
async def try_luck():
    return {"win": True}
