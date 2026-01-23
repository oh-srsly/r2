from email_validator import EmailNotValidError
from fastapi import FastAPI, HTTPException, Request

from backend.models import LoginResponse, LoginRequest
from backend.persistence import Persistence
import backend.services as services


persistence = Persistence()

app = FastAPI()

def validate_auth(request: Request) -> str:
    """
    :param request: The incoming HTTP request object containing the headers.
    :type request: Request
    :raises HTTPException: 401 HTTP error is raised if the token is missing or invalid.
    :return: The token
    :rtype: str
    """
    auth_header = request.headers.get("Authorization")

    if not auth_header or not auth_header.startswith("Bearer "):
        raise HTTPException(status_code=401, detail="Invalid authorization header")

    token = auth_header.replace("Bearer ", "")

    if not persistence.contains_token(token):
        raise HTTPException(status_code=401, detail="Invalid token")
    return token

@app.get("/health")
async def health_check():
    return {"status": "ok"}

@app.post("/api/login", response_model=LoginResponse)
async def login(payload: LoginRequest) -> LoginResponse:
    """
    Attempts login; returns token or raises HTTP error
    :raises HTTPException:
    :param payload:
    :type payload: LoginRequest
    :rtype: LoginResponse
    """
    try:
        try:
            normalized_email = services.validate_email(payload.email)
        except EmailNotValidError as e:
            raise HTTPException(status_code=400, detail=str(e))

        if not services.validate_password(normalized_email, payload.password):
            raise HTTPException(status_code=401, detail="Invalid password")

        token = services.generate_auth_token()
        persistence.add_token(token)
        return LoginResponse(token=token)

    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

@app.get("/api/logout", response_model=str)
async def logout(request: Request) -> str:
    """
    Log out user by invalidating the session token
    :raises HTTPException: if no valid authentication header is provided
    :param request:
    :type request: Request
    :return: str
    """
    token: str = validate_auth(request)
    persistence.remove_token(token)
    return "OK"

@app.get("/api/try_luck", response_model=dict[str, bool])
async def try_luck(request: Request) -> dict:
    """
    Attempts to win a prize; returns win status
    :return: dict[str, bool]
    """
    validate_auth(request)
    if services.try_luck(persistence.get_today_wins()):
        persistence.register_win()
        return {"win": True}
    else:
        return {"win": False}
