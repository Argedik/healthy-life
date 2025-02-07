[](https://charts.bitnami.com/)
$ helm repo add bitnami https://charts.bitnami.com/bitnami
helm repo update

# 1) Yeni namespace oluştur
sudo microk8s kubectl create namespace postgresql

# 2) Helm Chart kur
sudo microk8s helm3 install postgres-release bitnami/postgresql \
  --namespace postgresql \
  --set auth.postgresPassword='ArgedikPostgres123' \
  --set auth.username='argedik' \
  --set auth.password='ArgedikUser123'

> auth.postgresPassword super kullanici parolasi yani kullanici adi postgres olursa 
> argedik icin parola auth.password



[^1]: ==namespace postgresql== alani istedigimiz isimle degisir
https://kubernetes.io/docs/tasks/administer-cluster/namespaces-walkthrough/

[^2]: ==postgres-release== alani istedigimiz isimle degisir
https://github.com/bitnami/charts/tree/main/bitnami/postgresql