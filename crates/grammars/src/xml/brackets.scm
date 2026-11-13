((STag "<" @open ">" @close)
  (#set! rainbow.exclude))

((ETag "</" @open ">" @close)
  (#set! rainbow.exclude))

((EmptyElemTag "<" @open "/>" @close)
  (#set! rainbow.exclude))

("(" @open ")" @close)

("[" @open "]" @close)
