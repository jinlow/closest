from __future__ import annotations

class KDTree:
    """Simple KDTree Implementation"""

    def __init__(
        self, points: list[tuple[str | int | float, list[float]]], min_points: int = 1
    ): ...
    def get_nearest_neighbors(
        self, point: list[float], k: int = 1
    ) -> list[tuple[str | int | float, float]]:
        """Get k nearest neighbors."""
        ...
