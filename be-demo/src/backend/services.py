import secrets
import email_validator
import random

PASSWORD = "r2isthebest"
HIGH_WIN_RATE = 0.7
REDUCED_WIN_RATE = 0.4
WIN_RATE_REDUCTION_THRESHOLD = 30  # number of wins at the high rate


def validate_email(email: str) -> str:
    """
    Validates an email address using the email-validator library.
    Raises an error if the email is invalid.
    """
    try:
        # Validate the email address and get its normalized form
        validated = email_validator.validate_email(
            email, check_deliverability=False
        )  # Set check_deliverability=True for DNS checks
        email_address = validated.normalized  # Use the normalized email address
        return email_address
    except email_validator.EmailNotValidError as e:
        raise e


def validate_password(normalized_email: str, password: str) -> bool:
    return password == PASSWORD


def generate_auth_token() -> str:
    return secrets.token_hex(32)


def try_luck(wins_today: int) -> bool:
    if wins_today < WIN_RATE_REDUCTION_THRESHOLD:
        return random.random() < HIGH_WIN_RATE
    else:
        return random.random() < REDUCED_WIN_RATE
