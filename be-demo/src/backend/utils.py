import re
from email_validator import validate_email, EmailNotValidError

def is_valid_email(email) -> tuple[bool, str]:
    """
    Validates an email address using the email-validator library.
    Raises an error if the email is invalid.
    """
    try:
        # Validate the email address and get its normalized form
        validated = validate_email(email, check_deliverability=False) # Set check_deliverability=True for DNS checks
        email_address = validated.email # Use the normalized email address
        return True, email_address
    except EmailNotValidError as e:
        return False, str(e)

if __name__ == "__main__":
# Test cases
    print(f"'example@example.com' is valid: {is_valid_email('example@example.com')}")
    print(f"'invalid-email' is valid: {is_valid_email('invalid-email')}")
