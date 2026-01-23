import secrets
import email_validator

PASSWORD = "r2isthebest"


def validate_email(email) -> str:
    """
    Validates an email address using the email-validator library.
    Raises an error if the email is invalid.
    """
    try:
        # Validate the email address and get its normalized form
        validated = email_validator.validate_email(email, check_deliverability=False)  # Set check_deliverability=True for DNS checks
        email_address = validated.normalized  # Use the normalized email address
        return email_address
    except email_validator.EmailNotValidError as e:
        raise e


def validate_password(normalized_email, password):
    return password == PASSWORD


def generate_auth_token():
    return secrets.token_hex(32)
