import secrets
from email_validator import validate_email, EmailNotValidError

PASSWORD = "r2isthebest"


def is_valid_email(email) -> str:
    """
    Validates an email address using the email-validator library.
    Raises an error if the email is invalid.
    """
    try:
        # Validate the email address and get its normalized form
        validated = validate_email(email, check_deliverability=False)  # Set check_deliverability=True for DNS checks
        email_address = validated.normalized  # Use the normalized email address
        return email_address
    except EmailNotValidError as e:
        raise e


def validate_password(normalized_email, password):
    return password == PASSWORD


def generate_and_store_auth_token():
    return secrets.token_hex(32)
