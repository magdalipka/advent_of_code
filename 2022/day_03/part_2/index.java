import java.util.Scanner;

public class index {
  public static Scanner scan = new Scanner(System.in);

  public static String commonChars(String a, String b) {
    String result = "";
    for (char c : a.toCharArray()) {
      if (b.contains(c + "")) {
        result += c;
      }
    }
    return result;
  }

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
      points += charToInt(commonChar(scan.nextLine(), commonChars(scan.nextLine(), scan.nextLine())));
    }
    System.out.println(points);
  }
}
