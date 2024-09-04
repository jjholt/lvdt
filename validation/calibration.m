%% Load data and initialise parameters
data=csvread('calibration/1.csv');
time = data(2:end, 1);
voltage = data(2:end, 2) * 1000;  
len = length(time);

%% Instron calibration
instron_displacement_rate= 10; % um/s

%% Filter
fs = 2400; 
cutoff_freq = 100;
[b,a] = butter(2, 100/(fs/2));
voltage = filter(b,a,voltage);

% for file_number = 2:5
%     filename = sprintf('calibration/%d.csv', file_number);
%     data = csvread(filename);
%     voltage = [voltage (data(2:len+1, 2) * 1000)];  
% end


voltage_avg = mean(voltage,2);
chunk_size = 10;
num_chunks = floor(length(voltage_avg) / chunk_size);

avg_values = zeros(1, num_chunks);
max_values = zeros(1, num_chunks);
min_values = zeros(1, num_chunks);
chunk_time = zeros(1, num_chunks);

%% Create chunks
for i = 1:num_chunks
    start_idx = (i-1) * chunk_size + 1;
    end_idx = i * chunk_size;
    chunk = voltage_avg(start_idx:end_idx);
    
    avg_values(i) = mean(chunk);
    max_values(i) = max(chunk);
    min_values(i) = min(chunk);
    chunk_time(i) = mean(time(start_idx:end_idx));
end

% %% Plotting
% figure;
% hold on;
%
% plot(chunk_time, avg_values, '-o', 'Color', 'blue', 'LineWidth', 2);
%
% plot(chunk_time, max_values, '-o', 'Color', 'red', 'LineWidth', 2);
% plot(chunk_time, min_values, '-o', 'Color', 'red', 'LineWidth', 2);
%
% xlabel('Time (s)'); ylabel('Voltage (mV)');
% title('Averages, Max, and Min of Voltage (mV) over Time');
% legend('Average', 'Maxima');
% grid on;
%
% hold off;

%% Starting point, where there's no voltage change
% Find index to where 1.5 sec has elapsed. Should be made flexible, but whatever
target = 1.5;
tolerance = 0.05;
indices = find(abs(chunk_time - target) < tolerance); 
i = 1:indices(1);
data = [avg_values(i)' max_values(i)' min_values(i)'];
data = data - min(min_values(i));
csvwrite('calibration/variance.csv', [chunk_time(i)' data] );


%% Voltage starts changing
ii = indices(2);
voltage = (max(avg_values(ii:end)) - (avg_values(ii:end)))';
voltage = voltage(voltage < max(voltage) - 2); % Anything 2 mV of peak is considered peak.
shifted_time = chunk_time(ii:end) - chunk_time(ii);

% Fit line
x = shifted_time(1:length(voltage))'; 
y = voltage;
coeff = polyfit(x,y,1);
yfit = polyval(coeff, x);

um_per_mv = instron_displacement_rate/coeff(1);
csvwrite('um_per_mv.csv', um_per_mv);

csvwrite('calibration/data.csv', [shifted_time(1:length(voltage))' voltage yfit])
plot(x, y, 'b.', 'MarkerSize', 15); % Plot training data.
hold on; % Set hold on so the next plot does not blow away the one we just drew.
plot(x, yfit, 'r-', 'LineWidth', 2); % Plot fitted line.
grid on;
hold off;
