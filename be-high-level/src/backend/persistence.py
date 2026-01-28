import logging
from datetime import date, datetime, timezone, tzinfo
from collections import defaultdict

logger = logging.getLogger(__name__)
DAY_CHANGE_TIMEZONE: tzinfo = timezone.utc


class Persistence:
    def __init__(self):
        self._sessions: set[str] = set()
        self._timezone: tzinfo = DAY_CHANGE_TIMEZONE
        self._daily_wins: dict[date, int] = defaultdict(int)

    def add_token(self, token: str) -> None:
        self._sessions.add(token)

    def remove_token(self, token: str) -> bool:
        try:
            self._sessions.remove(token)
            return True
        except KeyError as e:
            logger.exception(e)
            return False

    def contains_token(self, token: str) -> bool:
        return token in self._sessions

    def _today(self) -> date:
        return datetime.now(tz=self._timezone).date()

    def get_today_wins(self) -> int:
        return self._daily_wins.get(self._today(), 0)

    def register_win(self) -> None:
        self._daily_wins[self._today()] += 1
