if [ $# -ne 1 ]; then
  echo "Usage: $0 [type]"
  exit 1
fi

type="$1"

files=("CMT1" "CMT2" "CMT3" "CMT4" "CMT5" "CMT11" "CMT12")

for file_name in "${files[@]}"; do
    file_path=".vrp/$file_name/$type.txt" 


# solve ./.vrp/CMT4/qbsolv/CMT4_11.vrp end: 14.124896

# Verwenden Sie den regulären Ausdruck, um die Zahlen zu finden
numbers=$(cat $file_path | grep -E -o 'solve [^ ]* end: [0-9]+\.[0-9]+' | grep -E -o '[0-9]+\.[0-9]+')

# Initialisieren Sie die Summe und die Anzahl der Zahlen
sum=0
count=0

# Durchlaufen Sie die gefundenen Zahlen und berechnen Sie die Summe und die Anzahl
for number in $numbers; do
    sum=$(awk "BEGIN {print $sum + $number}")
    count=$((count + 1))
done

# Berechnen Sie den Durchschnitt
average_solve=$(awk "BEGIN {print $sum / $count}")


# hybrid post transform ./.vrp/CMT4/qbsolv/CMT4_11.vrp end: 0.000088

# Verwenden Sie den regulären Ausdruck, um die Zahlen zu finden
numbers=$(cat $file_path | grep -E -o 'hybrid post transform [^ ]* end: [0-9]+\.[0-9]+' | grep -E -o '[0-9]+\.[0-9]+')

# Initialisieren Sie die Summe und die Anzahl der Zahlen
sum=0
count=0

# Durchlaufen Sie die gefundenen Zahlen und berechnen Sie die Summe und die Anzahl
for number in $numbers; do
    sum=$(awk "BEGIN {print $sum + $number}")
    count=$((count + 1))
done

# Berechnen Sie den Durchschnitt
average_post=$(awk "BEGIN {print $sum / $count}")


# hybrid qubo transform ./.vrp/CMT4/qbsolv/CMT4_11.vrp end: 0.05705

# Verwenden Sie den regulären Ausdruck, um die Zahlen zu finden
numbers=$(cat $file_path | grep -E -o 'hybrid qubo transform [^ ]* end: [0-9]+\.[0-9]+' | grep -E -o '[0-9]+\.[0-9]+')

# Initialisieren Sie die Summe und die Anzahl der Zahlen
sum=0
count=0

# Durchlaufen Sie die gefundenen Zahlen und berechnen Sie die Summe und die Anzahl
for number in $numbers; do
    sum=$(awk "BEGIN {print $sum + $number}")
    count=$((count + 1))
done

# Berechnen Sie den Durchschnitt
average_pre=$(awk "BEGIN {print $sum / $count}")


# connected after 0.01802206039428711


# Verwenden Sie den regulären Ausdruck, um die Zahlen zu finden
numbers=$(cat $file_path | grep -E -o 'connected after [0-9]+\.[0-9]+' | grep -E -o '[0-9]+\.[0-9]+')

# Initialisieren Sie die Summe und die Anzahl der Zahlen
sum=0
count=0

# Durchlaufen Sie die gefundenen Zahlen und berechnen Sie die Summe und die Anzahl
for number in $numbers; do
    sum=$(awk "BEGIN {print $sum + $number}")
    count=$((count + 1))
done

# Berechnen Sie den Durchschnitt
average_con=$(awk "BEGIN {print $sum / $count}")




# workflow created took 2.294981002807617

# Verwenden Sie den regulären Ausdruck, um die Zahlen zu finden
numbers=$(cat $file_path | grep -E -o 'workflow created took [0-9]+\.[0-9]+' | grep -E -o '[0-9]+\.[0-9]+')

# Initialisieren Sie die Summe und die Anzahl der Zahlen
sum=0
count=0

# Durchlaufen Sie die gefundenen Zahlen und berechnen Sie die Summe und die Anzahl
for number in $numbers; do
    sum=$(awk "BEGIN {print $sum + $number}")
    count=$((count + 1))
done

# Berechnen Sie den Durchschnitt
average_create=$(awk "BEGIN {print $sum / $count}")


# ended 13.087378978729248

# Verwenden Sie den regulären Ausdruck, um die Zahlen zu finden
numbers=$(cat $file_path | grep -E -o 'ended [0-9]+\.[0-9]+' | grep -E -o '[0-9]+\.[0-9]+')

# Initialisieren Sie die Summe und die Anzahl der Zahlen
sum=0
count=0

# Durchlaufen Sie die gefundenen Zahlen und berechnen Sie die Summe und die Anzahl
for number in $numbers; do
    sum=$(awk "BEGIN {print $sum + $number}")
    count=$((count + 1))
done

# Berechnen Sie den Durchschnitt
average_ended=$(awk "BEGIN {print $sum / $count}")


# connection closed after 1701725.182551

# Verwenden Sie den regulären Ausdruck, um die Zahlen zu finden
numbers=$(cat $file_path | grep -E -o 'connection closed after [0-9]+\.[0-9]+' | grep -E -o '[0-9]+\.[0-9]+')

# Initialisieren Sie die Summe und die Anzahl der Zahlen
sum=0
count=0

# Durchlaufen Sie die gefundenen Zahlen und berechnen Sie die Summe und die Anzahl
for number in $numbers; do
    sum=$(awk "BEGIN {print $sum + $number}")
    count=$((count + 1))
done

# Berechnen Sie den Durchschnitt
average_closed=$(awk "BEGIN {print $sum / $count}")



# 'qpu_access_time': 187530.0
texts=$(cat $file_path | tr '\n' ' ' | grep -Po '(?<!created took)([\S\s](?!created took))*qpu_access_time((?<!dispatch)[\S\s])*(?!dispatch)')

# Verwenden Sie den regulären Ausdruck, um die Zahlen zu finden
numbers=()

while IFS= read -r line; do
    times=$(echo "$line" | grep -E -o "'qpu_access_time': [0-9]+" | grep -E -o '[0-9]+')
    sum=0
    for number in $times; do
        sum=$(awk "BEGIN {print $sum + $number}")
    done
    numbers+=($sum)
done <<< "$texts"

# Initialisieren Sie die Summe und die Anzahl der Zahlen
sum=0
count=0

# Durchlaufen Sie die gefundenen Zahlen und berechnen Sie die Summe und die Anzahl
for number in $numbers; do
    sum=$(awk "BEGIN {print $sum + $number}")
    count=$((count + 1))
done

# Berechnen Sie den Durchschnitt
qpu_time=$(awk "BEGIN {print $sum / ($count * 1000000)}")


# "dispatch.next": 49999999

# Verwenden Sie den regulären Ausdruck, um die Zahlen zu finden
numbers=$(cat $file_path | grep -E -o '"dispatch.next": \[[0-9]+\.[0-9]+\]' | grep -E -o '[0-9]+\.[0-9]+')

# Initialisieren Sie die Summe und die Anzahl der Zahlen
sum=0
count=0

# Durchlaufen Sie die gefundenen Zahlen und berechnen Sie die Summe und die Anzahl
for number in $numbers; do
    sum=$(awk "BEGIN {print $sum + $number}")
    count=$((count + 1))
done

# Berechnen Sie den Durchschnitt
sampler_run_time=$(awk "BEGIN {print $sum / ($count)}")



clustertime=$(cat $file_path | grep -E -o -m 1 'clustered after: [0-9]+\.[0-9]+' | grep -E -o -m 1 '[0-9]+\.[0-9]+')
time=$(cat $file_path | grep -E -o -m 1 'finished: [0-9]+\.[0-9]+' | grep -E -o -m 1 '[0-9]+\.[0-9]+')
quality=$(cat $file_path | grep -E -o -m 1 'length: [0-9]+\.[0-9]+' | grep -E -o -m 1 '[0-9]+\.[0-9]+')

clusterno=$(grep -o 'QUBO Solver: started' "$file_path" | wc -l)

echo "Durchschnitt: $quality;$time;$average_create;$average_con;$average_pre;$average_post;$average_solve;$average_closed;$average_ended;$qpu_time;$sampler_run_time;$clusterno;$clustertime"

done