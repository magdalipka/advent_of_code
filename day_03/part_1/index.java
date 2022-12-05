import java.util.Scanner;

public class index {
  public static Scanner scan = new Scanner(System.in);

  public static char commonChar(String a, String b) {
    for (char c : a.toCharArray()) {
      if (b.contains(c + "")) {
        return c;
      }
    }
    throw new Error("Invalid inputs");
  }

  public static int charToInt(char c) {
    if ((int) c < 97) {
      return (int) c - 38;
    } else {
      return (int) c - 96;
    }
  }

  public static void main(String[] args) {
    int points = 0;
    while (scan.hasNextLine()) {
      String line = scan.nextLine();
      String left = line.substring(0, line.length() / 2);
      String right = line.substring(left.length());
      points += charToInt(commonChar(left, right));
    }
    System.out.println(points);
  }
}
