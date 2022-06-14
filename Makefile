entities_dir=./src/database/entities

.PHONY: entities
entities: .env
	@rm -rf $(entities_dir)
	@sea-orm-cli generate entity --compact-format --with-serde both --output-dir $(entities_dir)