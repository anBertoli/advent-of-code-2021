
CHANGES=""

# Run this script in the exercise folder currently used.
fswatch --batch-marker=END_BATCH --recursive -e ".*" -i "\\.rs$" -x . | while read file events; do
   CHANGES="${CHANGES}${file} ${events}"
   if [ $file = "END_BATCH" ]; then
      clear
      echo $CHANGES
      CHANGES=""
      cargo run
   fi
done

