if [ $# -ne 1 ]; then
  echo "Usage: $0 [type]"
  exit 1
fi

type="$1"

files=("CMT1" "CMT2" "CMT3" "CMT4" "CMT5" "CMT11" "CMT12")

for file_name in "${files[@]}"; do
    file_path=".vrp/$file_name/$type.txt" 

clusterno=$(grep 'result' "$file_path" | grep -o '\[' | wc -l)
clusterno=$((clusterno - 1))

echo "Durchschnitt: $clusterno"

done