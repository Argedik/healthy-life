***Browser ile bağlantı sağlama***

kok dizine k8s/deployments/backend/pgadmin.yaml dosyasi olustur

pgadmin.yaml dosyasina asagidaki kodlari yaz.

apiVersion: apps/v1
kind: Deployment
metadata:
  name: pgadmin4
  namespace: postgresql
spec:
  replicas: 1
  selector:
    matchLabels:
      app: pgadmin4
  template:
    metadata:
      labels:
        app: pgadmin4
    spec:
      containers:
      - name: pgadmin4
        image: dpage/pgadmin4:latest
        ports:
        - containerPort: 80
        env:
        - name: PGADMIN_DEFAULT_EMAIL
          value: "gedikas@hotmail.com"     
        - name: PGADMIN_DEFAULT_PASSWORD
          value: "mysecurepass3436"
---
apiVersion: v1
kind: Service
metadata:
  name: pgadmin4-service
  namespace: postgresql
spec:
  type: NodePort
  selector:
    app: pgadmin4
  ports:
    - protocol: TCP
      port: 80
      targetPort: 80
      nodePort: 30080

---
ardindan pgadmin.yaml klasorunun dizininde olan terminale asagidaki kodu yaz 
sudo microk8s kubectl apply -f pgadmin.yaml

---

➜  backend git:(master) ✗ sudo microk8s kubectl get pods -A           
NAMESPACE     NAME                                         READY   STATUS    RESTARTS       AGE
kube-system   calico-kube-controllers-759cd8b574-v6p62     1/1     Running   3 (6h8m ago)   12h
kube-system   calico-node-t7v9j                            1/1     Running   3 (6h8m ago)   12h
kube-system   coredns-7896dbf49-bxkcs                      1/1     Running   3 (6h8m ago)   12h
kube-system   dashboard-metrics-scraper-6b96ff7878-2f7hf   1/1     Running   3 (6h8m ago)   12h
kube-system   hostpath-provisioner-5fbc49d86c-5pjfd        1/1     Running   0              26m
kube-system   kubernetes-dashboard-7d869bcd96-htj42        1/1     Running   3 (6h8m ago)   12h
kube-system   metrics-server-d6f74bb9f-2dht8               1/1     Running   3 (6h8m ago)   12h
postgresql    pgadmin4-5d6b687fb6-gklqp                    1/1     Running   0              5m37s
postgresql    postgres-release-postgresql-0                1/1     Running   0              11m
➜  backend git:(master) ✗ sudo microk8s kubectl get pods -n postgresql
NAME                            READY   STATUS    RESTARTS   AGE
pgadmin4-5d6b687fb6-gklqp       1/1     Running   0          5m40s
postgres-release-postgresql-0   1/1     Running   0          11m


hangi service veya IP 
microk8s kubectl get svc -n postgresql



***Localde bağlantı sağlama***
- sudo apt install curl wget ca-certificates gnupg -y
- 1. Add pgAdmin4 repository
- curl  -fsSL https://www.pgadmin.org/static/packages_pgadmin_org.pub | sudo gpg --dearmor -o /etc/apt/trusted.gpg.d/pgadmin.gpg
- sudo sh -c 'echo "deb https://ftp.postgresql.org/pub/pgadmin/pgadmin4/apt/$(lsb_release -cs) pgadmin4 main" > /etc/apt/sources.list.d/pgadmin4.list'
YÜklenip yüklenmediğini teyit ediyoruz.
- cat /etc/apt/sources.list.d/pgadmin4.list
- Kaynak: https://computingforgeeks.com/how-to-install-pgadmin-4-on-ubuntu/

- 2. Install pgAdmin4 on Ubuntu
- sudo apt update
- sudo apt install pgadmin4


***Kurulum kontrolleri***
- which psql ==Dosya yolunu gösterir==
- psql --version ==Yüklüyse sürüm numarası==
- sudo systemctl status postgresql =="active (running)" görünüyorsa sorunsuz çalışıyordur.== exited görünüyorsa aşağıdaki komutlar çalışıyorsa sorun yok demektir.

sudo -u postgres psql ==Eğer bu komut aşağıdaki gibi çıktı veriyorsa postgresqlimiz sorunsuz çalışıyor demektir.==
psql (15.10)
Type "help" for help.

postgres=#

***Postgresql komutları***

\l ==Var olan tüm veri tabanlarını listeler==
Default hali;
`
                                  List of databases
   Name    |  Owner   | Encoding | Collate |  Ctype  |   Access privileges
-----------+---------+----------+---------+---------+-----------------------
 postgres  | postgres | UTF8     | en_US.  | en_US.  |
 template0 | postgres | UTF8     | en_US.  | en_US.  | =c/postgres          +
           |          |          |         |         | postgres=CTc/postgres
 template1 | postgres | UTF8     | en_US.  | en_US.  | =c/postgres          +
           |          |          |         |         | postgres=CTc/postgres
(3 rows)
`

