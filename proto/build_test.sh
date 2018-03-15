rootdir=`dirname $0`

echo $rootdir


while read include package name; do
  prefix="$rootdir/proto/$package"
  include="$rootdir/proto/$include"

  echo $prefix
  echo $include
  echo $name
  find "$prefix" -name "*.proto" | xargs protoc -I "$include" -o "$rootdir/$name.desc"
done
