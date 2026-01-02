"""Configuration management utilities."""

import os
import yaml
from pathlib import Path
from typing import Any, Dict, Optional
from dotenv import load_dotenv


class Config:
    """Configuration manager for the application."""

    def __init__(self, config_path: Optional[str] = None):
        """
        Initialize configuration.

        Args:
            config_path: Path to YAML configuration file
        """
        if config_path is None:
            config_path = os.path.join(
                Path(__file__).parent.parent.parent,
                "config",
                "config.yaml"
            )

        self.config_path = Path(config_path)
        self.config: Dict[str, Any] = {}
        self._load_config()
        self._load_env_vars()

    def _load_config(self) -> None:
        """Load configuration from YAML file."""
        if self.config_path.exists():
            with open(self.config_path, 'r') as f:
                self.config = yaml.safe_load(f) or {}
        else:
            raise FileNotFoundError(
                f"Configuration file not found: {self.config_path}"
            )

    def _load_env_vars(self) -> None:
        """Load environment variables from .env file."""
        env_path = self.config_path.parent / ".env"
        if env_path.exists():
            load_dotenv(env_path)

        # Override API keys with environment variables if present
        if 'api_keys' in self.config:
            if os.getenv('EXA_API_KEY'):
                self.config['api_keys']['exa'] = os.getenv('EXA_API_KEY')
            if os.getenv('BETTAFISH_API_KEY'):
                self.config['api_keys']['bettafish'] = os.getenv('BETTAFISH_API_KEY')
            if os.getenv('ALPHA_VANTAGE_API_KEY'):
                self.config['api_keys']['alpha_vantage'] = os.getenv('ALPHA_VANTAGE_API_KEY')

    def get(self, key: str, default: Any = None) -> Any:
        """
        Get configuration value by dot-notation key.

        Args:
            key: Configuration key (supports dot notation, e.g., 'api_keys.exa')
            default: Default value if key not found

        Returns:
            Configuration value
        """
        keys = key.split('.')
        value = self.config

        for k in keys:
            if isinstance(value, dict) and k in value:
                value = value[k]
            else:
                return default

        return value

    def get_api_key(self, service: str) -> str:
        """
        Get API key for a service.

        Args:
            service: Service name (e.g., 'exa', 'bettafish')

        Returns:
            API key

        Raises:
            ValueError: If API key not found
        """
        api_key = self.get(f'api_keys.{service}')
        if not api_key or api_key.startswith('your-'):
            raise ValueError(
                f"API key for '{service}' not found. "
                f"Please set it in config/config.yaml or .env file."
            )
        return api_key

    def get_assets(self) -> list:
        """Get list of assets to track."""
        return self.get('data.assets', [])

    def get_paths(self) -> Dict[str, str]:
        """Get all configured paths."""
        base_path = self.config_path.parent.parent
        paths = self.get('paths', {})

        # Convert relative paths to absolute
        for key, value in paths.items():
            if not os.path.isabs(value):
                paths[key] = os.path.join(base_path, value)

        return paths
