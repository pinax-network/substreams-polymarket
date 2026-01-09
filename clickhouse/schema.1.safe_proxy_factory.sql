-- SafeProxyFactory ProxyCreation --
CREATE TABLE IF NOT EXISTS safeproxyfactory_proxy_creation AS TEMPLATE_LOG
COMMENT 'SafeProxyFactory ProxyCreation events';
ALTER TABLE safeproxyfactory_proxy_creation
	-- event information --
	ADD COLUMN IF NOT EXISTS proxy               String COMMENT 'Proxy address',
	ADD COLUMN IF NOT EXISTS singleton           String COMMENT 'Singleton address';

-- SafeProxyFactory ProxyCreationL2 --
CREATE TABLE IF NOT EXISTS safeproxyfactory_proxy_creation_l2 AS TEMPLATE_LOG
COMMENT 'SafeProxyFactory ProxyCreationL2 events';
ALTER TABLE safeproxyfactory_proxy_creation_l2
	-- event information --
	ADD COLUMN IF NOT EXISTS proxy               String COMMENT 'Proxy address',
	ADD COLUMN IF NOT EXISTS singleton           String COMMENT 'Singleton address',
	ADD COLUMN IF NOT EXISTS initializer         String COMMENT 'Initializer data',
	ADD COLUMN IF NOT EXISTS salt_nonce          String COMMENT 'Salt nonce';

-- SafeProxyFactory ChainSpecificProxyCreationL2 --
CREATE TABLE IF NOT EXISTS safeproxyfactory_chain_specific_proxy_creation_l2 AS TEMPLATE_LOG
COMMENT 'SafeProxyFactory ChainSpecificProxyCreationL2 events';
ALTER TABLE safeproxyfactory_chain_specific_proxy_creation_l2
	-- event information --
	ADD COLUMN IF NOT EXISTS proxy               String COMMENT 'Proxy address',
	ADD COLUMN IF NOT EXISTS singleton           String COMMENT 'Singleton address',
	ADD COLUMN IF NOT EXISTS initializer         String COMMENT 'Initializer data',
	ADD COLUMN IF NOT EXISTS salt_nonce          String COMMENT 'Salt nonce',
	ADD COLUMN IF NOT EXISTS chain_id            String COMMENT 'Chain ID';
