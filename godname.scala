// Copyright mako, All Rights Reserved
package controllers
import util.Random

class nameDiviner(r:Random){
  def apply():String ={
    apply(8 - r.nextInt(4))
  }
  def apply(length:Int):String ={
    type Platter = Seq[(Float,String)]
    def gen(v:Platter): ()=>String = {
      def len(in:Platter, running:Float):Float = 
        in match{
          case (w,_) :: tail => len(tail, running + w)
          case Nil => running
        }
      val platterLength = len(v,0)
      () => {
        val index = r.nextFloat*(platterLength - 0.001f)
        def seek(i:Float, in:Platter):String =
          in match {
            case (w, str) :: tail =>
              if(i <= w) str
              else seek(i - w, tail)
            case Nil => "heaven" //unobtainable
          }
        seek(index, v)
      }
    }
    val middle = gen(Seq(
      (1.5f, "ll"),
      (1f, "mn"),
      (1f, "m"),
      (2.3f, "z"),
      (1.2f, "n"),
      (1.2f, "p"),
      (1f, "d"),
      (2f, "rh"),
      (1.1f, "g"),
      (1.3f, "r")))
    val alone = gen(Seq(
      (1f, "ts"),
      (1f, "s"),
      (1f, "r"),
      (1.5f, "h"),
      (0.8f, "th"),
      (0.8f, "sh"),
      (0.3f, "j"),
      (1.1f, "n"),
      (1f, "l")))
    val vowel = gen(Seq(
      (3.2f,"a"),
      (3.2f,"e"),
      (2.6f,"u"),
      (2.9f,"o"),
      (0.9f,"ae"),
      (0.5f,"ea"),
      (0.5f,"eu")))
    val cEnding = gen(Seq(
      (1.2f,"es"), (1.2f,"os"), (1f,"on"), (1.2f, "")))
    var res = cEnding
    var count = 0
    def fromVowel(total:String):String ={
      count = count + 1
      if(count >= length-1 || r.nextBoolean)
        fromConsonant(alone() + total)
      else
        fromMiddle(middle() + total)
    }
    def fromConsonant(total:String):String ={ // Alone
      count = count + 1
      if(count < length)
        fromVowel(vowel() + total)
      else
        total
    }
    def fromMiddle(total:String):String ={
      count = count + 1
      fromVowel(vowel() + total)
    }
            
    fromVowel(cEnding())
  }
}
