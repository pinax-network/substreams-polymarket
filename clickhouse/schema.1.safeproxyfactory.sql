-- SafeProxyFactory ProxyCreation --
CREATE TABLE IF NOT EXISTS safeproxyfactory_proxy_creation AS TEMPLATE_LOG
COMMENT 'SafeProxyFactory ProxyCreation events';
ALTER TABLE safeproxyfactory_proxy_creation
	-- event information --
	ADD COLUMN IF NOT EXISTS proxy                String COMMENT 'Proxy address',
	ADD COLUMN IF NOT EXISTS owner                String COMMENT 'Owner address';
