import unittest
from backend.utils import is_valid_email

class TestEmailValidator(unittest.TestCase):

    def test_valid_email(self):
        """
        Tests a valid email address.
        """
        is_valid, message = is_valid_email("test@example.com")
        self.assertTrue(is_valid)
        self.assertEqual(message, "test@example.com")

    def test_invalid_email(self):
        """
        Tests an invalid email address.
        """
        is_valid, message = is_valid_email("invalid-email")
        self.assertFalse(is_valid)
        self.assertIn("An email address must have an @-sign.", message)

    def test_email_with_whitespace(self):
        """
        Tests an email address with leading/trailing whitespace.
        """
        is_valid, message = is_valid_email("  test@example.com  ")
        self.assertFalse(is_valid)
        self.assertIn("The email address contains invalid characters before the @-sign: SPACE.", message)

    def test_email_with_subdomain(self):
        """
        Tests an email address with a subdomain.
        """
        is_valid, message = is_valid_email("test@mail.example.com")
        self.assertTrue(is_valid)
        self.assertEqual(message, "test@mail.example.com")

    def test_empty_email(self):
        """
        Tests an empty string as an email.
        """
        is_valid, message = is_valid_email("")
        self.assertFalse(is_valid)
        self.assertIn("An email address must have an @-sign.", message)

if __name__ == '__main__':
    unittest.main()
