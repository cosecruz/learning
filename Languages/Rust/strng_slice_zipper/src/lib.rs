//What do i have
//What changes
//When do i stop

/**
 * What do i have(state)?
 *      N sentences
 *      For each sentence, I create an iterator over its words
 *      Each iterator remembers where it is internally
 *      I store all iterators in a list
 */

/**
  * what changes
  * In each round, I try to pull one word from each iterator
If an iterator is exhausted, I skip it
If at least one word was pulled, progress happened
  */

/**
   * When do i stop?
   * If I loop over all sentences
and none of them produced a word
then I am done
   */

/**
   * Invariant
   * The result contains all words pulled so far,
in round-robin order,
and no word appears twice.
   */

/**
   * get sentences
convert each sentence into a word iterator
store all iterators in a list

result = []

loop:
    made_progress = false

    for each iterator:
        if iterator has a next word:
            push word to result
            made_progress = true

    if made_progress == false:
        break

   */

fn zip_sentences<'a>(sentences: &'a [&'a str]) -> Vec<&'a str> {
    let mut result = Vec::new();

    //turn each sentence into an iterator
    let mut iterators: Vec<_> = sentences.iter().map(|s| s.split_whitespace()).collect();

    loop {
        let mut progressed = false;

        for it in iterators.iter_mut() {
            if let Some(word) = it.next() {
                result.push(word);
                progressed = true;
            }
        }

        if !progressed {
            break;
        }
    }

    result
}

//for unzip
//
//what i have
//  zipped words
//  number of sources
//  lengths of each sources
//  current index into zipped list

//what to changes each step
// - Take the next word from zipped
// - Assign it to the current source
// - Decrement remaining length for that source
// - Move to next source that still has remaining capacity

//when do i stop
// when zipped list is exhausted

//invariant
// - Total remaining lengths == remaining zipped words
// - No source exceeds its expected length
// - Order within each source is preserved

//simulation
// - zipped = [a,1,x,b,2,y,c,z,d,w]
// - lengths = [4,2,4]

// source 0 ← a
// source 1 ← 1
// source 2 ← x
// source 0 ← b
// source 1 ← 2 (source 1 now done)
// source 2 ← y
// source 0 ← c
// source 2 ← z
// source 0 ← d (source 0 done)
// source 2 ← w
