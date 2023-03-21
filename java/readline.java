// Java program to demonstrate BufferedReader
import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
public class readline {
	public static void main(String[] args)
		throws IOException
	{
		// Enter data using BufferReader
		BufferedReader reader = new BufferedReader(
			new InputStreamReader(System.in));

		int x = 0;
		while (reader.readLine() != null) {
			x++ ;
		}

		// Printing the read line
		System.out.println(x);
	}
}

