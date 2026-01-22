import unittest
from backend.utils import is_valid_email
from email_validator import EmailNotValidError

class TestEmailValidator(unittest.TestCase):

    def test_valid_email(self):
        """
        Tests a valid email address.
        """
        email = is_valid_email("test@example.com")
        self.assertEqual(email, "test@example.com")

    def test_invalid_email(self):
        """
        Tests an invalid email address.
        """
        with self.assertRaises(EmailNotValidError):
            is_valid_email("invalid-email")

    def test_email_with_whitespace(self):
        """
        Tests an email address with leading/trailing whitespace.
        """
        with self.assertRaises(EmailNotValidError):
            is_valid_email("  test@example.com  ")

    def test_email_with_subdomain(self):
        """
        Tests an email address with a subdomain.
        """
        email = is_valid_email("test@mail.example.com")
        self.assertEqual(email, "test@mail.example.com")

    def test_empty_email(self):
        """
        Tests an empty string as an email.
        """
        with self.assertRaises(EmailNotValidError):
            is_valid_email("")

if __name__ == '__main__':
    unittest.main()
