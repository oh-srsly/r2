import unittest
from unittest.mock import patch
from fastapi.testclient import TestClient
from email_validator import EmailNotValidError

from backend.main import app
from backend.models import TryLuckResponse, LoginResponse


class TestMain(unittest.TestCase):

    def setUp(self):
        self.client = TestClient(app)

    def test_health_check(self):
        response = self.client.get("/health")
        self.assertEqual(response.status_code, 200)

    @patch('backend.main.services')
    @patch('backend.main.persistence')
    def test_login_success(self, mock_persistence, mock_services):
        mock_services.validate_email.return_value = "test@example.com"
        mock_services.validate_password.return_value = True
        mock_services.generate_auth_token.return_value = "test_token"

        response = self.client.post("/api/login", json={"email": "test@example.com", "password": "password"})

        self.assertEqual(response.status_code, 200)
        self.assertEqual(LoginResponse.model_validate(response.json()), LoginResponse(token="test_token"))
        mock_persistence.add_token.assert_called_with("test_token")

    @patch('backend.main.services')
    def test_login_invalid_email(self, mock_services):
        mock_services.validate_email.side_effect = EmailNotValidError("Invalid email")

        response = self.client.post("/api/login", json={"email": "invalid-email", "password": "password"})

        self.assertEqual(response.status_code, 400)

    @patch('backend.main.services')
    def test_login_invalid_password(self, mock_services):
        mock_services.validate_email.return_value = "test@example.com"
        mock_services.validate_password.return_value = False

        response = self.client.post("/api/login", json={"email": "test@example.com", "password": "wrong_password"})

        self.assertEqual(response.status_code, 401)

    @patch('backend.main.persistence')
    def test_logout_success(self, mock_persistence):
        mock_persistence.contains_token.return_value = True
        
        response = self.client.get("/api/logout", headers={"Authorization": "Bearer test_token"})
        
        self.assertEqual(response.status_code, 200)
        mock_persistence.remove_token.assert_called_with("test_token")

    def test_logout_no_token(self):
        response = self.client.get("/api/logout")
        self.assertEqual(response.status_code, 401)

    @patch('backend.main.persistence')
    def test_logout_invalid_token(self, mock_persistence):
        mock_persistence.contains_token.return_value = False
        
        response = self.client.get("/api/logout", headers={"Authorization": "Bearer invalid_token"})
        
        self.assertEqual(response.status_code, 401)

    @patch('backend.main.services')
    @patch('backend.main.persistence')
    def test_try_luck_win(self, mock_persistence, mock_services):
        mock_persistence.contains_token.return_value = True
        mock_persistence.get_today_wins.return_value = 0
        mock_services.try_luck.return_value = True

        response = self.client.get("/api/try_luck", headers={"Authorization": "Bearer test_token"})

        self.assertEqual(response.status_code, 200)
        self.assertEqual(TryLuckResponse.model_validate(response.json()), TryLuckResponse(win=True))
        mock_persistence.register_win.assert_called_once()

    @patch('backend.main.services')
    @patch('backend.main.persistence')
    def test_try_luck_lose(self, mock_persistence, mock_services):
        mock_persistence.contains_token.return_value = True
        mock_persistence.get_today_wins.return_value = 10
        mock_services.try_luck.return_value = False

        response = self.client.get("/api/try_luck", headers={"Authorization": "Bearer test_token"})

        self.assertEqual(response.status_code, 200)
        self.assertEqual(TryLuckResponse.model_validate(response.json()), TryLuckResponse(win=False))
        mock_persistence.register_win.assert_not_called()

    def test_try_luck_no_token(self):
        response = self.client.get("/api/try_luck")
        self.assertEqual(response.status_code, 401)

    @patch('backend.main.persistence')
    def test_try_luck_invalid_token(self, mock_persistence):
        mock_persistence.contains_token.return_value = False
        
        response = self.client.get("/api/try_luck", headers={"Authorization": "Bearer invalid_token"})
        
        self.assertEqual(response.status_code, 401)

if __name__ == '__main__':
    unittest.main()
