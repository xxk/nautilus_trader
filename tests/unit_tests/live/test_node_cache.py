# -------------------------------------------------------------------------------------------------
#  Copyright (C) 2015-2026 Nautech Systems Pty Ltd. All rights reserved.
#  https://nautechsystems.io
#
#  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
#  you may not use this file except in compliance with the License.
#  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
#
#  Unless required by applicable law or agreed to in writing, software
#  distributed under the License is distributed on an "AS IS" BASIS,
#  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
#  See the License for the specific language governing permissions and
#  limitations under the License.
# -------------------------------------------------------------------------------------------------

from unittest.mock import Mock
from unittest.mock import patch

from nautilus_trader.common import Environment
from nautilus_trader.config import CacheConfig
from nautilus_trader.config import DataEngineConfig
from nautilus_trader.config import DatabaseConfig
from nautilus_trader.config import ExecEngineConfig
from nautilus_trader.config import LoggingConfig
from nautilus_trader.config import NautilusKernelConfig
from nautilus_trader.config import RiskEngineConfig
from nautilus_trader.system.kernel import NautilusKernel


def test_kernel_uses_postgres_cache_adapter_when_configured() -> None:
    # Arrange
    config = NautilusKernelConfig(
        environment=Environment.BACKTEST,
        trader_id="TRADER-001",
        cache=CacheConfig(
            database=DatabaseConfig(
                type="postgres",
                host="localhost",
                port=5432,
                username="nautilus",
                password="pass",
                database="nautilus",
            ),
        ),
        data_engine=DataEngineConfig(),
        risk_engine=RiskEngineConfig(),
        exec_engine=ExecEngineConfig(),
        logging=LoggingConfig(bypass_logging=True),
    )

    adapter = Mock()

    # Act
    with patch("nautilus_trader.system.kernel.CachePostgresAdapter", return_value=adapter) as adapter_ctor:
        kernel = NautilusKernel(name="TestKernel", config=config)

    # Assert
    adapter_ctor.assert_called_once_with(config=config.cache)
    kernel.dispose()