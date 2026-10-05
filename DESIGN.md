# Reconciliation
1. fetch + rebase
2. 署名済み?
3. `<tag_name>` を含む?
4. Build Phase
  1. Docker in Docker で `<container_image>` を起動
  2. `/input/` に `<root>` をコピー
  3. `<build-command>` を実行
  4. `/output/` にある成果物をコピー
5. Kubo に 再帰的に成果物をアップロード
6. pin
7. 登記
  1. ENS mode
  更新用のリンクを送る
  https://update-resolver.afabl.eth/update?resolver=0x...&contenthash=ipfs://...
  or
  勝手に指定した wallet から gas 支払い
  2. IPNS mode
  指定した秘密鍵で更新
  3. DNS mode
  Cloudflare API 経由で DNSLink を設定

# 定期実行
- Kubernetes CronJob
- 15min おきくらい

